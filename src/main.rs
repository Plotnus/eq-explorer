#![feature(associated_type_bounds)]
use petgraph::graph::Graph;
use petgraph::Direction::{Incoming,Outgoing};



type MyGraph = Graph<f32,()>;
type Names = Vec<&'static str>;
type NodeUpdateFn = dyn Fn(&mut MyGraph, &Names);
fn main() {
    let mut node_names: Names= Vec::new();
    let mut node_update_fns: Vec<&NodeUpdateFn> = Vec::new();

    let mut graph = Graph::<f32,()>::new();// what are these &str for?
    
    // px node
    let px = graph.add_node(42.);
    let px_name = "px";
    let px_fn = move |graph: &mut MyGraph, names: &Names| {
        println!("updating node \"{}\" to new_value: {}",
            names[px.index()],
            graph[px],
        );
    }; 
    node_names.push(px_name);
    node_update_fns.push(&px_fn);

    // sz node
    let sz = graph.add_node(100.0);
    let sz_name = "sz";
    let sz_fn = move |graph: &mut MyGraph, names: &Names| {
        println!("updating node \"{}\" to new_value: {}",
            names[sz.index()],
            graph[sz],
        );
    }; 
    node_names.push(sz_name);
    node_update_fns.push(&sz_fn);
    
    // pos_cost
    let pos_cost = graph.add_node(0.);
    let pos_cost_name = "Position Cost";
    let pos_cost_fn = move |graph: &mut MyGraph, names: &Names| {
        let node = pos_cost;
        let node_name = names[node.index()];
        let old_value = graph[node];
        let new_value = graph[px] * graph[sz];
        graph[node] = graph[px] * graph[sz];
        println!("updated node \"{}\": {} ->  {}",
            node_name,
            old_value,
            new_value,
        );
    };
    node_names.push(pos_cost_name);
    node_update_fns.push(&pos_cost_fn);
    


    node_update_fns[px.index()](&mut graph,&node_names);
    node_update_fns[sz.index()](&mut graph,&node_names);
    node_update_fns[pos_cost.index()](&mut graph,&node_names);
    

    // Ok, we have three nodes and pos_cost = px * sz;
    // now how to make it automatically update when mutated?
//    let sz = graph.add_node(100.);
//    node_names.push("sz");
//    node_update_fns.push(&|graph, names| {
//        println!("updating node \"{}\" to new_value: {}",
//            names[sz.index()],
//            graph[sz],
//        );
//    });
//    assert_eq!(px.index(), 0);
    

    
    //node_update_fns[px.index()](&graph,&node_names);

//    let b = graph.add_node("b"
//    );
//    let c = graph.add_node("c"
//    );
//    let d = graph.add_node("d"
//    );
//    let e = graph.add_node("e"
//    );
//    let f = graph.add_node("f"
//    );
//    graph.extend_with_edges(&[
//        (f,e),(f,d),
//        (e,a),(e,b),(e,c),
//    ]);
    

/*
    // Print Dependencies
    print!("Dependencies for node {}: [", graph[e]);
    let mut neighbors_iter = graph.neighbors(e).detach();
    while let Some(node) = neighbors_iter.next_node(&graph) {
        print!("{},", graph[node]);
    }
    println!("]");

    // Maybe can do `neigmbors_directed(node, <Incoming|Out)
    // Maybe can do `neigmbors_directed(node, <Incoming|Outgoing)
    // can be found in petgraph::Direction
    print!("Incoming for node {}: [", graph[e]);
    let mut neighbors_iter = graph.neighbors_directed(e, Incoming).detach();
    while let Some(node) = neighbors_iter.next_node(&graph) {
        print!("{},", graph[node]);
    }
    println!("]");
    println!();

    // Maybe can do `neigmbors_directed(node, <Incoming|Out)
    // Maybe can do `neigmbors_directed(node, <Incoming|Outgoing)
    // can be found in petgraph::Direction
    print!("Outgoing for node {}: [", graph[e]);
    let mut neighbors_iter = graph.neighbors_directed(e, Outgoing).detach();
    while let Some(node) = neighbors_iter.next_node(&graph) {
        print!("({},{}),", node.index(),graph[node]);
    }
    println!("]");
    println!();
    */
}