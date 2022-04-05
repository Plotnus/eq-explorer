#![feature(associated_type_bounds)]
use petgraph::graph::{Graph,NodeIndex};
use petgraph::Direction::{Incoming};
use std::collections::HashMap;


type MyGraph = Graph<(),()>;
struct NodeSpecification {
    name: String,
    initial_value: f32,
    depends_on: Vec<&'static str>,
}
struct SomeGraph {
    graph: MyGraph, // used to track dependencies... is invisible outside of module
    values: HashMap<NodeIndex, f32>,
    names: HashMap<NodeIndex, String>,
    ids: HashMap<String,NodeIndex>,
}
impl SomeGraph {
    fn new() -> Self {
        Self {
            graph: MyGraph::new(),
            //update_fns: HashMap::new(),
            values: HashMap::new(),
            names: HashMap::new(),
            ids: HashMap::new(),
        }
    }
    
    fn add_nodes(&mut self, nodes: &Vec<NodeSpecification>) {
        // first lets create all the nodes, then we'll connect them
        for n in nodes {
            let id = self.graph.add_node(());
            self.names.insert(id, n.name.clone());
            self.values.insert(id, n.initial_value);
            self.ids.insert(n.name.clone(), id);
        }
        // now lets add edges for who the node depends on
        for n in nodes {
            for &dependency in &n.depends_on {
                self.graph.add_edge(
                    self.ids[dependency], 
                    self.ids[&n.name],
                    (),
                );
            }
        }
    }
    
    fn print_dependencies_for(&self, name: &str) {
        print!("Dependencies for {}: ", name);
        let node_id = self.ids[name];
        let mut neighbors_iter = self.graph.neighbors_directed(node_id, Incoming).detach();
        while let Some(node) = neighbors_iter.next_node(&self.graph) {
            print!("{},", self.names[&node]);
        }
        println!();
    }
}


fn main() {
    let mut my_graph = SomeGraph::new();
    let node_specs = vec![
        NodeSpecification {
            name: "a".to_string(),
            initial_value: 13.,
            depends_on: vec![],
        },
        NodeSpecification {
            name: "b".to_string(),
            initial_value: 12.0,
            depends_on: vec![],
        },
        NodeSpecification {
            name: "c".to_string(),
            initial_value: 42.0,
            depends_on: vec![],
        },
        NodeSpecification {
            name: "d".to_string(),
            initial_value: 64.0,
            depends_on: vec!["a","b"],
        },
        NodeSpecification {
            name: "e".to_string(),
            initial_value: 32.0,
            depends_on: vec!["b","d","c"],
        },
    ];
    my_graph.add_nodes(&node_specs);
    my_graph.print_dependencies_for("e");
}