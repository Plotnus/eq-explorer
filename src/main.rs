#![feature(associated_type_bounds)]
use petgraph::graph::{Graph,NodeIndex};
use petgraph::Direction::{Incoming};
use std::collections::HashMap;
use std::rc::Rc;


type UpdateFn = Rc<dyn Fn(&mut HashMap<NodeIndex,f32>, HashMap<String,NodeIndex>)>;
type Value = f32;
struct NodeSpecification {
    name: String,
    initial_value: f32,
    depends_on: Vec<&'static str>,
    update_fn: Option<UpdateFn>, // leaf nodes do not have an update fn for recalculating
}

struct MyGraphBuilder {
    nodes: Vec<NodeSpecification>,
}
impl MyGraphBuilder {
    fn new() -> Self {
        MyGraphBuilder { nodes: vec![]}
    }
    fn add_nodes(mut self, mut nodes: Vec<NodeSpecification>) -> Self {
        self.nodes.append(&mut nodes);
        self
    }

    fn build(self) -> SomeGraph {
        type DependencyGraph = Graph<(),f32>;
        let mut graph: DependencyGraph = Graph::new();

        let mut out = SomeGraph::new();
        // first lets create all the nodes, then we'll connect them
        for n in &self.nodes {
            let id = graph.add_node(());
            out.names.insert(id, n.name.clone());
            out.values.insert(id, n.initial_value);
            out.ids.insert(n.name.clone(), id);
            if let Some(update_fn) = &n.update_fn {
                out.update_fns.insert(id, update_fn.clone());
            }
        }
        // now lets add edges for who the node depends on
        for n in &self.nodes {
            for &dependency in &n.depends_on {
                graph.add_edge(
                    out.ids[dependency], 
                    out.ids[&n.name],
                    //(),
                    -1., // we do this weight so we can update based on longest path to
                );
            }
        }
        // now that we have our graph, lets determine the updates need when input nodes change
        // input_node - a node that can be updated
        for i in 0..out.ids.len() {
            let node_index = NodeIndex::new(i);
            let path = petgraph::algo::bellman_ford(&graph, node_index).unwrap();
            let mut max_distances = 
                path.distances
                    .iter()
                    .enumerate()
                    .filter(|(_,&v)| v < 0.) // since (-) node weight filters for 
                    .map(|(i,&v)| (v.abs(),i)) // so we can sort by value
                    .collect::<Vec<(f32,usize)>>();
            // sort the index by distance
            max_distances.sort_by(|a,b| a.partial_cmp(b).unwrap());
            let update_path: Vec<NodeIndex> = {
                max_distances.iter()
                    .map(|(_,i)| NodeIndex::new(*i))
                    .collect::<Vec<NodeIndex>>()
            };
            out.update_path.insert(node_index, update_path);
        }
        out
    }
}
struct SomeGraph {
    values: HashMap<NodeIndex, Value>, // stores the value of each node. Not much gained keeping it out of graph
    names: HashMap<NodeIndex, String>, // keeping it out of graph allows us to print stuff about graph
    ids: HashMap<String,NodeIndex>, // name -> id
    update_fns: HashMap<NodeIndex,UpdateFn>,
    update_path: HashMap<NodeIndex,Vec<NodeIndex>>, // update order for consistence
}
impl SomeGraph {
    fn new() -> Self {
        Self {
            update_fns: HashMap::new(),
            update_path: HashMap::new(),
            values: HashMap::new(),
            names: HashMap::new(),
            ids: HashMap::new(),
        }
    }
    
    fn print_update_order_for(&self, name: &str) {
        // TODO: have handle case/error of name being incorrect
        print!("Dependencies for {}: ", name);
        let node_id = self.ids[name];
        if let Some(nodes) = self.update_path.get(&node_id) {
            for node_id in nodes {
                print!("{}", self.names[node_id]);
            }
        }
        println!();
    }
}


fn main() {
    let node_specifications = vec![
        NodeSpecification {
            name: "a".to_string(),
            initial_value: 13.,
            depends_on: vec![],
            update_fn: None,
        },
        NodeSpecification {
            name: "b".to_string(),
            initial_value: 12.0,
            depends_on: vec![],
            update_fn: None,
        },
        NodeSpecification {
            name: "c".to_string(),
            initial_value: 42.0,
            depends_on: vec![],
            update_fn: None,
        },
        NodeSpecification {
            name: "d".to_string(),
            initial_value: 64.0,
            depends_on: vec!["a","b"],
            update_fn: Some(Rc::new(
                |values,ids| {
                    let a = values[&ids["a"]];
                    let b = values[&ids["b"]];
                    let d = a * b;
                    values.insert(ids["d"], d);
            })),
        },
        NodeSpecification {
            name: "e".to_string(),
            initial_value: 32.0,
            depends_on: vec!["b","d","c"],
            update_fn: Some(Rc::new(
                |values,ids| {
                    let b = values[&ids["b"]];
                    let d = values[&ids["d"]];
                    let c = values[&ids["c"]];
                    let e = b * d * c;
                    values.insert(ids["e"], e);
            })),
        },
    ];
    let my_graph = {
        MyGraphBuilder::new()
            .add_nodes(node_specifications)
            .build()
    };
    
    my_graph.print_update_order_for("a");
    my_graph.print_update_order_for("b");
    my_graph.print_update_order_for("c");
    my_graph.print_update_order_for("d");
    my_graph.print_update_order_for("e");
}