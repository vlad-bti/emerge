#[macro_use]
extern crate lazy_static;
extern crate regex;

mod data;
mod depgraph;
mod ebuild_utils;

use crate::depgraph::{init_dep_graph, DepGraph, GraphData};
use std::env;

fn help() {
    println!(
        "usage:
        emerge <string>"
    );
}

fn build_dag(mut package_name_list: Vec<String>) -> Result<GraphData, String> {
    let mut graph = init_dep_graph();

    let node_name_s = "s";
    let node_name_t = "t";

    graph.add_node(node_name_s)?;
    graph.add_node(node_name_t)?;

    for package_name in &package_name_list {
        graph.add_node(package_name)?;
        graph.add_edge(package_name, node_name_t)?;
    }

    while !package_name_list.is_empty() {
        let package_name = package_name_list.pop().unwrap();

        let package_info = ebuild_utils::load_package_info(&package_name)?;

        let package_version = package_info.version_list.first().unwrap();
        if package_version.depends_list.is_empty() {
            graph.add_edge(node_name_s, &package_name)?;
        }

        for depend_name in package_version.depends_list.iter() {
            graph.add_node(depend_name)?;
            graph.add_edge(depend_name, &package_name)?;
            package_name_list.push(depend_name.into());
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

    if graph.is_cyclic_directed() {
        return Err(String::from("directed graph contains a cycle"));
    }

    Ok(graph)
}

fn print_dag(graph: GraphData) {
    let order = graph.toposort();
    println!("{:?}", order);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let result = build_dag(args[1..].into());
    if let Err(e) = result {
        println!("error: {:?}", e);
        return;
    }

    print_dag(result.unwrap());
}
