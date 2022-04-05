use petgraph::graph::{Graph,NodeIndex};
use std::collections::HashMap;
use std::rc::Rc;


type UpdateFn = Rc<dyn Fn(&mut Vec<f32>, &HashMap<String,usize>)>;
type Value = f32;
pub struct NodeSpecification {
    pub name: String,
    pub initial_value: f32,
    pub depends_on: Vec<&'static str>,
    pub update_fn: Option<UpdateFn>, // leaf nodes do not have an update fn for recalculating
}

pub struct MyGraphBuilder {
    nodes: Vec<NodeSpecification>,
}
impl MyGraphBuilder {
    pub fn new() -> Self {
        MyGraphBuilder { nodes: vec![]}
    }
    pub fn add_nodes(mut self, mut nodes: Vec<NodeSpecification>) -> Self {
        self.nodes.append(&mut nodes);
        self
    }

    pub fn build(self) -> ExpressionTree {
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
pub struct ExpressionTree {
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
    
    pub fn update_value(&mut self, node_name: &str, value: Value) {
        let i = match self.ids.get(node_name) {
            Some(&index) => index,
            _ => {
                println!("No value {} found", node_name);
                return;
            }
        };
        // TODO: add check that this is a leaf node?
        println!("Updating \"{}\" from {} to {}", node_name, self.values[i], value);
        self.values[i] = value;
        for &i in &self.update_paths[i] {
            let old_value = self.values[i];

            if let Some(f) = &self.update_fns[i] {
                f(&mut self.values, &self.ids);
            }
            let new_value = self.values[i];
            println!("Updated \"{}\" from {:.2} to {:.2}", self.names[i], old_value, new_value);
        }
    }
    
    pub fn print(&self) {
        print!("NodeValues: ");
        for i in 0..self.names.len() {
            print!("({},{}),", self.names[i], self.values[i]);
        }
        println!();
    }
}