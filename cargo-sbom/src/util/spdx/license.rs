use anyhow::{anyhow, Ok, Result};

pub fn normalize_license_string<S: AsRef<str>>(
  rust_license_string: S,
) -> Result<String> {
  let license_expr = spdx::Expression::parse_mode(
    rust_license_string.as_ref(),
    spdx::ParseMode::LAX,
  )?;

  // License expression is stored in postfix order, so convert to an infix license string
  // We maintain two stacks to do this: one storing the string being built, the other
  // storing operator information, used to associate operators with parenthesis when required (based on operator precedence)
  let mut string_stack = Vec::<String>::new();
  let mut op_stack = Vec::<Option<spdx::expression::Operator>>::new();

  for node in license_expr.iter() {
    match node {
      spdx::expression::ExprNode::Req(req) => {
        string_stack.push(req.req.license.to_string());
        op_stack.push(None);
      }
      spdx::expression::ExprNode::Op(spdx::expression::Operator::Or) => {
        let a = string_stack.pop().ok_or_else(|| {
          anyhow!(
            "Failed to parse license string: {}",
            rust_license_string.as_ref()
          )
        })?;
        let b = string_stack.pop().ok_or_else(|| {
          anyhow!(
            "Failed to parse license string: {}",
            rust_license_string.as_ref()
          )
        })?;
        op_stack.pop().ok_or_else(|| {
          anyhow!(
            "Failed to parse license string: {}",
            rust_license_string.as_ref()
          )
        })?;
        op_stack.pop().ok_or_else(|| {
          anyhow!(
            "Failed to parse license string: {}",
            rust_license_string.as_ref()
          )
        })?;

        op_stack.push(Some(spdx::expression::Operator::Or));
        string_stack.push(format!("{} OR {}", b, a));
      }
      spdx::expression::ExprNode::Op(spdx::expression::Operator::And) => {
        let mut a = string_stack.pop().ok_or_else(|| {
          anyhow!(
            "Failed to parse license string: {}",
            rust_license_string.as_ref()
          )
        })?;
        let mut b = string_stack.pop().ok_or_else(|| {
          anyhow!(
            "Failed to parse license string: {}",
            rust_license_string.as_ref()
          )
        })?;
        let a_op = op_stack.pop().ok_or_else(|| {
          anyhow!(
            "Failed to parse license string: {}",
            rust_license_string.as_ref()
          )
        })?;
        let b_op = op_stack.pop().ok_or_else(|| {
          anyhow!(
            "Failed to parse license string: {}",
            rust_license_string.as_ref()
          )
        })?;

        // AND takes precedence, so parenthesize the OR expressions before applying AND
        if matches!(a_op, Some(spdx::expression::Operator::Or)) {
          a = format!("({})", a);
        }
        if matches!(b_op, Some(spdx::expression::Operator::Or)) {
          b = format!("({})", b);
        }

        op_stack.push(Some(spdx::expression::Operator::And));
        string_stack.push(format!("{} AND {}", b, a));
      }
    }
  }
  Ok(string_stack.pop().ok_or_else(|| {
    anyhow!(
      "Failed to parse license string: {}",
      rust_license_string.as_ref()
    )
  })?)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_or() {
    assert_eq!(
      normalize_license_string("MIT OR Apache-2.0").unwrap(),
      "MIT OR Apache-2.0"
    );

    assert_eq!(
      normalize_license_string("MIT / Apache-2.0").unwrap(),
      "MIT OR Apache-2.0"
    );

    assert_eq!(
      normalize_license_string("MIT / Apache-2.0").unwrap(),
      "MIT OR Apache-2.0"
    );
  }

  #[test]
  fn test_and() {
    assert_eq!(
      normalize_license_string("MIT AND Apache-2.0").unwrap(),
      "MIT AND Apache-2.0"
    );
  }

  #[test]
  fn test_precedence() {
    assert_eq!(
      normalize_license_string("MIT AND Apache-2.0 OR BSD-2-Clause").unwrap(),
      "MIT AND Apache-2.0 OR BSD-2-Clause"
    );

    assert_eq!(
      normalize_license_string("MIT AND (Apache-2.0 OR BSD-2-Clause)").unwrap(),
      "MIT AND (Apache-2.0 OR BSD-2-Clause)"
    );

    assert_eq!(
      normalize_license_string("(MIT AND Apache-2.0) OR BSD-2-Clause").unwrap(),
      "MIT AND Apache-2.0 OR BSD-2-Clause"
    );

    assert_eq!(
      normalize_license_string("Apache-2.0 OR BSD-2-Clause AND MIT").unwrap(),
      "Apache-2.0 OR BSD-2-Clause AND MIT"
    );

    assert_eq!(
      normalize_license_string("Apache-2.0 OR (BSD-2-Clause AND MIT)").unwrap(),
      "Apache-2.0 OR BSD-2-Clause AND MIT"
    );

    assert_eq!(
      normalize_license_string("(Apache-2.0 OR BSD-2-Clause) AND MIT").unwrap(),
      "(Apache-2.0 OR BSD-2-Clause) AND MIT"
    );
  }
}
