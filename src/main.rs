use petgraph::prelude::*;

use std::env;

fn help() {
    println!("usage:
        emerge <string>");
}

fn build_dag(packages_org: Vec<String>) -> Result<DiGraphMap::<&'static str, i8>, String> {
    let mut graph = DiGraphMap::<&str, i8>::new();
    let node_s = graph.add_node("s");
    let node_t = graph.add_node("t");

    let list = packages_org.to_vec();

    while !list.is_empty() {
        let package_name = list.pop();

        parse_ebuild(&package_name)?;

        if packages.contains(&package_name) {
            check_restrictions()?;
            merge_restrictions();
        } else {
            packages.add(&package_name);
            graph.add_node(&package_name);
        }

        if packages_org.contains(&package_name) {
            graph.add_edge(&package_name, &node_t, 1);
        }
        if package_depends.is_empty() {
            graph.add_edge(&node_s, &package_name, 1);
        }

        list.extend_from_slice(package_depends.as_slice());
    }

    if !petgraph::algo::is_cyclic_directed(&graph) {
        return Err("directed graph contains a cycle");
    }

    Ok(graph)
}

fn print_dag(dag: DiGraphMap::<&'static str, i8>) {
    let order = petgraph::algo::toposort(&dag, None).unwrap();
    println!("{:?}", order);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let result = build_dag(args[1..].to_vec());
    if let Err(e) = result {
        println!("error: {:?}", e);
        return;
    }

    print_dag(result.unwrap());
}
