#[macro_use]
extern crate lazy_static;
extern crate regex;

mod data;

use petgraph::prelude::*;

use std::env;

mod ebuild_utils;

fn help() {
    println!(
        "usage:
        emerge <string>"
    );
}

fn build_dag(mut package_name_list: Vec<&str>) -> Result<DiGraphMap<&str, i8>, String> {
    let mut graph = DiGraphMap::<&str, i8>::new();
    let node_s = graph.add_node("s");
    let node_t = graph.add_node("t");

    for package_name in &package_name_list {
        graph.add_node(&package_name);
        graph.add_edge(&package_name, &node_t, 1);
    }

    while !package_name_list.is_empty() {
        let package_name = package_name_list.pop().unwrap();

        let package_info = ebuild_utils::load_package_info(&package_name)?;

        let package_version = package_info.version_list.first().unwrap();
        if package_version.depends_list.is_empty() {
            graph.add_edge(&node_s, &package_name, 1);
        }

        for depend_name in package_version.depends_list {
            graph.add_node(&depend_name);
            graph.add_edge(&depend_name, &package_name, 1);
            package_name_list.push(&depend_name);
        }
        /*
                if package_info_list.contains(&package_name) {
                    package_info_list.check_restrictions(&package_info)?;
                    package_info_list.merge_restrictions(&package_info);
                } else {
                    package_info_list.push(package_info);
                }
        */
    }

    if petgraph::algo::is_cyclic_directed(&graph) {
        return Err(String::from("directed graph contains a cycle"));
    }

    Ok(graph)
}

fn print_dag(dag: DiGraphMap<&str, i8>) {
    let order = petgraph::algo::toposort(&dag, None).unwrap();
    println!("{:?}", order);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let result = build_dag(args[1..].iter().map(|arg| arg.as_str()).collect());
    if let Err(e) = result {
        println!("error: {:?}", e);
        return;
    }

    print_dag(result.unwrap());
}
