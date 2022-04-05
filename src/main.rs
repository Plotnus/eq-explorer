#![feature(associated_type_bounds)]
use petgraph::graph::{Graph,NodeIndex};
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

    fn build(self) -> ExpressionTree {
        type DependencyGraph = Graph<(),f32>;
        let mut graph: DependencyGraph = Graph::new();

        let mut out = ExpressionTree::new();
        // first lets create all the nodes, then we'll connect them
        for (i,n) in self.nodes.iter().enumerate() {
            let id = graph.add_node(());
            assert_eq!(i,id.index());
            out.ids.insert(n.name.clone(), id.index());
            out.names.push(n.name.clone());
            out.values.push(n.initial_value);
            out.update_fns.push(n.update_fn.clone());
        }
        // now lets build the edges for the dependency graph we're using to
        // calculate the update order for each node which we then cache
        for n in &self.nodes {
            for &dependency in &n.depends_on {
                graph.add_edge(
                    NodeIndex::new(out.ids[dependency]), 
                    NodeIndex::new(out.ids[&n.name]),
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
            let update_path: Vec<usize> = {
                max_distances.iter()
                    .map(|(_,i)| *i)
                    .collect()
            };
            out.update_paths.push(update_path);
        }
        out
    }
}
struct ExpressionTree {
    // useful for going from node name -> node index
    ids: HashMap<String,usize>,
    // stores the values of each node
    values: Vec<Value>,
    // stores the names of each node
    names: Vec<String>,

    // stores the instructions for how to update each node when it is changed
    update_fns: Vec<Option<UpdateFn>>,

    update_paths: Vec<Vec<usize>>, // update order for consistence
}
impl ExpressionTree {
    fn new() -> Self {
        Self {
            update_fns: Vec::new(),
            update_paths: Vec::new(),
            values: Vec::new(),
            names: Vec::new(),
            ids: HashMap::new(),
        }
    }
    
    fn print_update_order_for(&self, name: &str) {
        // TODO: have handle case/error of name being incorrect
        print!("on change to {}, we update: ", name);
        let node_index = self.ids[name];
        for &i in &self.update_paths[node_index] {
            print!("{}, ", &self.names[i]);
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