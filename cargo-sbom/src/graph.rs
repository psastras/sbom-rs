use anyhow::{anyhow, Error};
use cargo_metadata::{DependencyKind, Metadata, Package, PackageId};
use petgraph::graph::NodeIndex;
use petgraph::stable_graph::StableGraph;
use std::collections::HashMap;

pub struct Graph<'a> {
  pub graph: StableGraph<&'a Package, DependencyKind>,
  pub nodes: HashMap<PackageId, NodeIndex>,
  pub root_packages: Vec<PackageId>,
}

pub fn build<'a>(metadata: &'a Metadata) -> Result<Graph<'a>, Error> {
  let resolve = metadata.resolve.as_ref().unwrap();
  let mut graph: Graph<'a> = Graph {
    graph: StableGraph::new(),
    nodes: HashMap::new(),
    root_packages: vec![],
  };

  for package in metadata.workspace_packages() {
    let id = package.id.clone();
    graph.root_packages.push(id.clone());
  }

  for package in metadata.packages.iter() {
    let id = package.id.clone();
    let index = graph.graph.add_node(package);
    graph.nodes.insert(id, index);
  }

  for node in resolve.nodes.iter() {
    if node.deps.len() != node.dependencies.len() {
      return Err(anyhow!("cargo tree requires cargo 1.41 or newer"));
    }

    let from = graph.nodes[&node.id];
    for dep in node.deps.iter() {
      if dep.dep_kinds.is_empty() {
        return Err(anyhow!("cargo tree requires cargo 1.41 or newer"));
      }

      // https://github.com/rust-lang/cargo/issues/7752
      let mut kinds: Vec<DependencyKind> = vec![];
      for kind in dep.dep_kinds.iter() {
        if !kinds.contains(&kind.kind) {
          kinds.push(kind.kind);
        }
      }

      let to = graph.nodes[&dep.pkg];
      for kind in kinds {
        // skip dev dependencies
        if kind == DependencyKind::Development || kind == DependencyKind::Build
        {
          continue;
        }

        graph.graph.add_edge(from, to, kind);
      }
    }
  }

  Ok(graph)
}
