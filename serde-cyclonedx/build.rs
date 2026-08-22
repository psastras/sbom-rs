use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::path::PathBuf;

use anyhow::bail;
use anyhow::Context;
use anyhow::Ok;
use anyhow::Result;
use schemafy_lib::Expander;
use schemafy_lib::Schema;

// Types which come from subschemas that are not vendored in `schemas/`.
// schemafy names them after the reference but cannot resolve their definitions,
// so each one referenced by the schema is emitted as a string type alias.
const EXTERNAL_REF_TYPES: &[&str] = &["SpdxSchemaJson", "Signature"];

// Checks whether the generated code mentions `name` as a whole identifier,
// not merely as a substring of a longer one such as `SignatureAlgorithm`.
fn references_ident(generated: &str, name: &str) -> bool {
  generated
    .split(|c: char| !c.is_alphanumeric() && c != '_')
    .any(|token| token == name)
}

// Add additional items to the generated cyclonedx.rs file
// Currently adds: derive(Builder) to each struct
// and appropriate use statements at the top of the file
// todo: this (and other parts) need a refactor and tests
fn process_token_stream(input: proc_macro2::TokenStream) -> syn::File {
  let generated = input.to_string();
  let mut ast: syn::File = syn::parse2(input).unwrap();

  // add use directives to top of the file
  ast.items.insert(
    0,
    syn::parse_quote! {
      use serde::{Serialize, Deserialize};
    },
  );
  ast.items.insert(
    0,
    syn::parse_quote! {
      use derive_builder::Builder;
    },
  );

  for name in EXTERNAL_REF_TYPES {
    if references_ident(&generated, name) {
      let ident = proc_macro2::Ident::new(name, proc_macro2::Span::call_site());
      ast.items.insert(
        0,
        syn::parse_quote! {
          type #ident = String;
        },
      );
    }
  }

  // Checks if the type is an Option type (returns true if yes, false otherwise)
  fn path_is_option(path: &syn::Path) -> bool {
    let idents_of_path =
      path.segments.iter().fold(String::new(), |mut acc, v| {
        acc.push_str(&v.ident.to_string());
        acc.push('|');
        acc
      });

    vec!["Option|", "std|option|Option|", "core|option|Option|"]
      .into_iter()
      .any(|s| idents_of_path == *s)
  }

  // schemafy emits a second `serde(rename)` on an enum variant whose name
  // starts with a digit. Only the first one, which carries the value from the
  // schema, describes how the variant is serialized.
  fn deduplicate_renames(attrs: &mut Vec<syn::Attribute>) {
    let mut seen_rename = false;
    attrs.retain(|attr| {
      // `starts_with("rename =")` does not match `rename_all`
      let is_rename = attr.path().is_ident("serde")
        && match &attr.meta {
          syn::Meta::List(list) => {
            list.tokens.to_string().starts_with("rename =")
          }
          _ => false,
        };

      if is_rename {
        if seen_rename {
          return false;
        }
        seen_rename = true;
      }
      true
    });
  }

  ast.items.iter_mut().for_each(|ref mut item| {
    if let syn::Item::Enum(e) = item {
      e.variants
        .iter_mut()
        .for_each(|variant| deduplicate_renames(&mut variant.attrs));
    }

    if let syn::Item::Struct(s) = item {
      // add builder attributes to each struct
      s.attrs.extend(vec![
        syn::parse_quote! {
          #[derive(Builder)]
        },
        syn::parse_quote! {
          #[builder(setter(into, strip_option))]
        },
      ]);

      // for each struct field, if that field is Optional, set None
      // as the default value when using the builder
      (&mut s.fields).into_iter().for_each(|ref mut field| {
        if let syn::Type::Path(typepath) = &field.ty {
          if path_is_option(&typepath.path) {
            field.attrs.push(syn::parse_quote! {
              #[builder(setter(into, strip_option), default)]
            });
          }
        }
      });
    }
  });

  ast
}

// schemafy generates code from these
const STRUCTURAL_KEYWORDS: &[&str] = &[
  "properties",
  "$ref",
  "items",
  "type",
  "allOf",
  "anyOf",
  "oneOf",
];

// schemafy ignores these
const CONSTRAINT_KEYWORDS: &[&str] = &[
  "not",
  "if",
  "then",
  "else",
  "required",
  "description",
  "title",
];

// Removes `allOf` blocks built only from constraint keywords: schemafy would
// otherwise generate an empty struct, ignoring any sibling `properties`.
// Unknown keywords are an error — stripping on a guess could silently
// generate wrong types.
fn strip_constraint_only_all_of(value: &mut serde_json::Value) -> Result<()> {
  match value {
    serde_json::Value::Object(map) => {
      if let Some(subschemas) =
        map.get("allOf").and_then(serde_json::Value::as_array)
      {
        let structural = subschemas.iter().any(|subschema| {
          STRUCTURAL_KEYWORDS
            .iter()
            .any(|keyword| subschema.get(keyword).is_some())
        });

        if !structural {
          for subschema in subschemas {
            let keys = subschema
              .as_object()
              .map(|subschema| subschema.keys())
              .into_iter()
              .flatten();
            for key in keys {
              if !CONSTRAINT_KEYWORDS.contains(&key.as_str()) {
                bail!(
                  "`allOf` subschema uses the keyword `{key}`, which is \
                   neither a known structural nor a known constraint \
                   keyword; add it to the matching list in build.rs"
                );
              }
            }
          }
          map.remove("allOf");
        }
      }

      map.values_mut().try_for_each(strip_constraint_only_all_of)
    }
    serde_json::Value::Array(items) => {
      items.iter_mut().try_for_each(strip_constraint_only_all_of)
    }
    _ => Ok(()),
  }
}

// Inlines definitions referenced from a subschema vendored alongside the
// CycloneDx schema, rewriting `other.schema.json#/definitions/name` to
// `#/definitions/name`. schemafy resolves references against the root schema
// only, so a definition has to live there for a real type to be generated for
// it. References to a subschema which is not vendored in `schemas/` are left
// as they are and fall back to the aliases in `EXTERNAL_REF_TYPES`.
fn inline_external_definitions(root: &mut serde_json::Value) -> Result<()> {
  let mut inlined = serde_json::Map::new();
  collect_external_definitions(root, &mut inlined)?;

  if inlined.is_empty() {
    return Ok(());
  }

  let definitions = root
    .get_mut("definitions")
    .and_then(serde_json::Value::as_object_mut)
    .context("schema has no definitions to inline into")?;

  for (name, definition) in inlined {
    if let Some(existing) = definitions.get(&name) {
      if *existing != definition {
        bail!("inlined definition `{name}` collides with a root definition");
      }
    }
    definitions.insert(name, definition);
  }

  Ok(())
}

fn collect_external_definitions(
  value: &mut serde_json::Value,
  inlined: &mut serde_json::Map<String, serde_json::Value>,
) -> Result<()> {
  match value {
    serde_json::Value::Object(map) => {
      if let Some(ext_ref) = external_definition_ref(map) {
        if let Some(definition) = read_external_definition(&ext_ref)? {
          if let Some(existing) = inlined.get(&ext_ref.definition) {
            if *existing != definition {
              bail!(
                "definition `{}` of `{}` collides with a definition of the \
                 same name inlined from another schema",
                ext_ref.definition,
                ext_ref.file
              );
            }
          }
          inlined.insert(ext_ref.definition.clone(), definition);
          map.insert(
            "$ref".to_string(),
            serde_json::Value::String(format!(
              "#/definitions/{}",
              ext_ref.definition
            )),
          );
        }
      }

      map
        .values_mut()
        .try_for_each(|value| collect_external_definitions(value, inlined))
    }
    serde_json::Value::Array(items) => items
      .iter_mut()
      .try_for_each(|item| collect_external_definitions(item, inlined)),
    _ => Ok(()),
  }
}

// A reference into the definitions of another schema file
struct ExternalRef {
  file: String,
  definition: String,
}

fn external_definition_ref(
  map: &serde_json::Map<String, serde_json::Value>,
) -> Option<ExternalRef> {
  let ref_ = map.get("$ref")?.as_str()?;
  let (file, fragment) = ref_.split_once('#')?;
  let definition = fragment.strip_prefix("/definitions/")?;

  // an empty file part is a reference within this schema
  (!file.is_empty()).then(|| ExternalRef {
    file: file.to_string(),
    definition: definition.to_string(),
  })
}

fn read_external_definition(
  ext_ref: &ExternalRef,
) -> Result<Option<serde_json::Value>> {
  let path = PathBuf::from("schemas").join(&ext_ref.file);
  if !path.exists() {
    return Ok(None);
  }
  println!("cargo:rerun-if-changed={}", path.display());

  let schema: serde_json::Value =
    serde_json::from_str(&std::fs::read_to_string(&path)?)?;
  let definition = schema
    .get("definitions")
    .and_then(|definitions| definitions.get(&ext_ref.definition))
    .with_context(|| {
      format!(
        "`{}` has no definition `{}`",
        ext_ref.file, ext_ref.definition
      )
    })?
    .clone();

  // references inside an inlined definition would resolve against the wrong
  // schema, so reject them rather than generate the wrong type
  if contains_ref_key(&definition) {
    bail!(
      "definition `{}` of `{}` contains references and cannot be inlined",
      ext_ref.definition,
      ext_ref.file
    );
  }

  Ok(Some(definition))
}

// Checks whether a schema contains a `$ref` key at any nesting depth
fn contains_ref_key(value: &serde_json::Value) -> bool {
  match value {
    serde_json::Value::Object(map) => {
      map.contains_key("$ref") || map.values().any(contains_ref_key)
    }
    serde_json::Value::Array(items) => items.iter().any(contains_ref_key),
    _ => false,
  }
}

fn generate_schema(version_str: &str) -> Result<()> {
  println!("cargo:rerun-if-changed=schemas/cyclonedx_{version_str}.json",);
  let path_str = format!("schemas/cyclonedx_{version_str}.json",);
  let path = Path::new(&path_str);

  // Generate the Rust schema struct
  let json = std::fs::read_to_string(path).unwrap();
  let mut value: serde_json::Value = serde_json::from_str(&json)?;
  strip_constraint_only_all_of(&mut value)?;
  inline_external_definitions(&mut value)?;
  let schema: Schema = serde_json::from_value(value)?;
  let path_str = path.to_str().unwrap();
  let mut expander = Expander::new(Some("CycloneDx"), path_str, &schema);
  let generated = process_token_stream(expander.expand(&schema));

  // Write the struct to the $OUT_DIR/cyclonedx.rs file.
  let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
  let mut file =
    File::create(out_path.join(format!("cyclonedx_{version_str}.rs",)))?;
  file.write_all(prettyplease::unparse(&generated).as_bytes())?;
  Ok(())
}

fn main() -> Result<()> {
  generate_schema("1_4")?;
  generate_schema("1_5")?;
  generate_schema("1_6")?;
  generate_schema("1_7")?;

  Ok(())
}
