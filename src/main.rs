#![feature(associated_type_bounds)]
mod expression_tree;
use expression_tree::{NodeSpecification,MyGraphBuilder, ExpressionTree};
use std::rc::Rc;

fn main() {
    let mut expr_tree = build_expression_tree();
    
    expr_tree.print();
    
    expr_tree.update_value("a", 2.);
    expr_tree.print();
    expr_tree.update_value("b", 3.);
    expr_tree.print();
    expr_tree.update_value("c", 4.);
    expr_tree.print();
}

fn build_expression_tree() -> ExpressionTree {
    let node_specifications = vec![
        NodeSpecification {
            name: "a".to_string(),
            initial_value: 1.,
            depends_on: vec![],
            update_fn: None,
        },
        NodeSpecification {
            name: "b".to_string(),
            initial_value: 5.,
            depends_on: vec![],
            update_fn: None,
        },
        NodeSpecification {
            name: "c".to_string(),
            initial_value: 10.,
            depends_on: vec![],
            update_fn: None,
        },
        NodeSpecification {
            name: "d".to_string(),
            initial_value: 64.0,
            depends_on: vec!["a","b"],
            update_fn: Some(Rc::new(
                |values,ids| {
                    let a = values[ids["a"]];
                    let b = values[ids["b"]];
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
                    let b = values[ids["b"]];
                    let d = values[ids["d"]];
                    let c = values[ids["c"]];
                    let e = b * d * c;
                    values.insert(ids["e"], e);
            })),
        },
    ];

    MyGraphBuilder::new()
        .add_nodes(node_specifications)
        .build()
}