use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DependencySymbol {
    pub name: String,
    pub path: String,
}

impl DependencySymbol {
    pub fn depends_on(&self, other: &Self, strict: bool) -> bool {
        if strict {
            self == other
        } else {
            self.name == other.name
                && Path::new(&self.path).parent() == Path::new(&other.path).parent()
                && Path::new(&self.path).file_stem() == Path::new(&other.path).file_stem()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DependencyInfo {
    pub input_path: String,
    pub output_path: String,
    pub undefined: Vec<DependencySymbol>,
    pub defined: Vec<DependencySymbol>,
}

impl PartialEq for DependencyInfo {
    fn eq(&self, other: &Self) -> bool {
        self.input_path == other.input_path && self.output_path == other.output_path
    }
}

impl DependencyInfo {
    pub fn is_main(&self) -> bool {
        self.defined.iter().any(|s| s.name == "main")
    }
}

#[derive(Debug)]
pub struct DependencyGraph {
    pub nodes: Vec<DependencyInfo>,
    pub edges: Vec<Vec<usize>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            nodes: Vec::new(),
            edges: Vec::new(),
        }
    }

    pub fn add_node(&mut self, node: DependencyInfo) {
        self.nodes.push(node);
        self.edges.push(Vec::new());
    }

    pub fn build_dependency_edges(&mut self, strict: bool) {
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

pub fn read_dependencies(dependency_file: &Path) -> Result<Vec<DependencyInfo>, Box<dyn Error>> {
    let file = File::open(dependency_file)?;
    let reader = BufReader::new(file);
    let dependencies: Vec<DependencyInfo> = serde_json::from_reader(reader)?;
    Ok(dependencies)
}
