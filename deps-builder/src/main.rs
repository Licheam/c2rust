use clap::Parser;
use dot_generator::*;
use dot_structures::*;
use graphviz_rust::printer::{DotPrinter, PrinterContext};
use serde::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, Write};
use std::path::{Path, PathBuf};
use std::process;

#[derive(Debug, Parser)]
#[clap(
name = "deps-builder",
about = "Build C dependencies for C2Rust",
long_about = None,
trailing_var_arg = true)]
struct Args {
    /// Use strict dependency checking
    #[arg(long)]
    strict_depends: bool,
    /// Path to a file to with the dependency information
    #[arg(long, default_value = "./dependencies.json")]
    dependency_file: PathBuf,
    /// Path to a file to write the dependency graph to
    #[arg(long, default_value = "./dependencies.dot")]
    dependency_dot: PathBuf,
}

#[derive(Debug, Deserialize, PartialEq)]
struct DependencySymbol {
    name: String,
    path: String,
}

impl DependencySymbol {
    fn depends_on(&self, other: &Self, strict: bool) -> bool {
        if strict {
            self == other
        } else {
            self.name == other.name
                && Path::new(&self.path).parent() == Path::new(&other.path).parent()
                && Path::new(&self.path).file_stem() == Path::new(&other.path).file_stem()
        }
    }
}

#[derive(Debug, Deserialize)]
struct DependencyInfo {
    input_path: String,
    output_path: String,
    undefined: Vec<DependencySymbol>,
    defined: Vec<DependencySymbol>,
}

impl PartialEq for DependencyInfo {
    fn eq(&self, other: &Self) -> bool {
        self.input_path == other.input_path && self.output_path == other.output_path
    }
}

impl DependencyInfo {
    fn is_main(&self) -> bool {
        self.defined.iter().any(|s| s.name == "main")
    }
}

#[derive(Debug)]
struct DependencyGraph {
    nodes: Vec<DependencyInfo>,
    edges: Vec<Vec<usize>>,
}

impl DependencyGraph {
    fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    fn add_node(&mut self, node: DependencyInfo) {
        self.nodes.push(node);
        self.edges.push(Vec::new());
    }

    fn build_dependency_edges(&mut self, strict: bool) {
        for (i, node) in self.nodes.iter().enumerate() {
            for symbol in &node.undefined {
                self.nodes.iter().enumerate().for_each(|(j, n)| {
                    if !n.is_main() && n.defined.iter().any(|s| s.depends_on(symbol, strict)) {
                        self.edges[i].push(j);
                    }
                });
            }
        }
    }
}

fn read_dependencies(dependency_file: &Path) -> Result<Vec<DependencyInfo>, Box<dyn Error>> {
    let file = File::open(dependency_file)?;
    let reader = BufReader::new(file);
    let dependencies: Vec<DependencyInfo> = serde_json::from_reader(reader)?;
    Ok(dependencies)
}

fn main() {
    let args = Args::parse();
    let strict_depends = args.strict_depends;
    let dependency_file = args.dependency_file;
    let dependency_dot = args.dependency_dot;

    // Read dependencies from the dependency file
    let dependencies = read_dependencies(&dependency_file).unwrap_or_else(|e| {
        eprintln!(
            "Error reading dependencies from {}: {}",
            dependency_file.display(),
            e
        );
        process::exit(1);
    });

    let mut dependency_graph = DependencyGraph::new();

    // Build the dependency graph
    dependencies.into_iter().for_each(|dependency| {
        dependency_graph.add_node(dependency);
    });

    // Build the dependency graph
    dependency_graph.build_dependency_edges(strict_depends);

    // println!("Dependency Graph: {:#?}", dependency_graph);

    // Write the dependency graph to a dot file
    let mut dependency_dot_graph = Graph::DiGraph {
        id: Id::Plain(String::from("dependency_graph")),
        strict: true,
        stmts: vec![],
    };

    for (i, node) in dependency_graph.nodes.iter().enumerate() {
        if let Some(_) = node.defined.iter().find(|s| s.name == "main") {
            dependency_dot_graph.add_stmt(Stmt::Node(
                node!(i;attr!("color", "red"), attr!("label", (format!("\"{}\"", (Path::new(&node.output_path).file_name().unwrap().to_str().unwrap()))))),
            ));
        } else {
            dependency_dot_graph.add_stmt(Stmt::Node(
                node!(i;attr!("label", (format!("\"{}\"", (Path::new(&node.output_path).file_name().unwrap().to_str().unwrap()))))),
            ));
        }
    }

    for (i, edges) in dependency_graph.edges.iter().enumerate() {
        for j in edges {
            dependency_dot_graph.add_stmt(Stmt::Edge(edge!(node_id!(i) => node_id!(j))));
        }
    }

    println!(
        "{}",
        dependency_dot_graph.print(&mut PrinterContext::default())
    );

    let mut dot_file = File::create(&dependency_dot).unwrap_or_else(|e| {
        eprintln!(
            "Error creating dependency dot file {}: {}",
            dependency_dot.display(),
            e
        );
        process::exit(1);
    });

    match dot_file.write_all(
        dependency_dot_graph
            .print(&mut PrinterContext::default())
            .as_bytes(),
    ) {
        Ok(()) => (),
        Err(e) => panic!(
            "Unable to write dependencies to file {}: {}",
            dependency_dot.display(),
            e
        ),
    };
}
