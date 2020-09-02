use std::collections::HashMap;

use petgraph::prelude::*;

use crate::ebuild_utils;

#[derive(Default)]
pub struct GraphData {
    // TODO: Graph
    pub graph: DiGraphMap<i32, i8>,
    pub index_to_name: HashMap<i32, String>,
    pub name_to_index: HashMap<String, i32>,
}

impl GraphData {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn build_dag_from(org_package_name_list: &[String]) -> Result<GraphData, String> {
        let mut graph = Self::new();

        const NODE_NAME_S: &str = "s";
        const NODE_NAME_T: &str = "t";

        graph.add_node(NODE_NAME_S)?;
        graph.add_node(NODE_NAME_T)?;

        let mut package_name_list = Vec::new();
        for package_name in org_package_name_list {
            graph.add_node(package_name)?;
            package_name_list.push(String::from(package_name));
        }

        while !package_name_list.is_empty() {
            let package_name = package_name_list.pop().unwrap();
            let package_info = ebuild_utils::load_package_info(&package_name)?;

            let package_version = package_info.version_list.first().unwrap();
            if package_version.depends_list.is_empty() {
                graph.add_edge(NODE_NAME_S, &package_name)?;
            }

            for depend_name in &package_version.depends_list {
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

        for package_name in org_package_name_list {
            if !graph.is_outgoing_neighbors_exists(&package_name)? {
                graph.add_edge(&package_name, NODE_NAME_T)?;
            }
        }

        if graph.is_cyclic_directed() {
            return Err(String::from("directed graph contains a cycle"));
        }

        Ok(graph)
    }

    pub fn print_dag(&self) {
        let order = self.toposort();
        println!("{:?}", order);
    }
}

pub trait DepGraph {
    fn add_node(&mut self, node_name: &str) -> Result<(), String>;
    fn add_edge(&mut self, node_name_a: &str, node_name_b: &str) -> Result<(), String>;
    fn is_outgoing_neighbors_exists(&self, node_name: &str) -> Result<bool, String>;
    fn is_cyclic_directed(&self) -> bool;
    fn toposort(&self) -> Vec<String>;
}

impl DepGraph for GraphData {
    fn add_node(&mut self, node_name: &str) -> Result<(), String> {
        let count = self.graph.node_count() as i32;
        if !self.name_to_index.contains_key(node_name) {
            self.name_to_index.insert(node_name.into(), count);
            self.index_to_name.insert(count, node_name.into());
            self.graph.add_node(count);
        }

        Ok(())
    }

    fn add_edge(&mut self, node_name_a: &str, node_name_b: &str) -> Result<(), String> {
        if !self.name_to_index.contains_key(node_name_a) {
            return Err(format!("'{}' node don't exists", node_name_a));
        }
        if !self.name_to_index.contains_key(node_name_b) {
            return Err(format!("'{}' node don't exists", node_name_b));
        }

        let index_a = self.name_to_index.get(node_name_a).unwrap();
        let index_b = self.name_to_index.get(node_name_b).unwrap();

        if !self.graph.contains_edge(*index_a, *index_b) {
            self.graph.add_edge(*index_a, *index_b, 1);
        }
        Ok(())
    }

    fn is_outgoing_neighbors_exists(&self, node_name: &str) -> Result<bool, String> {
        if !self.name_to_index.contains_key(node_name) {
            return Err(format!("'{}' node don't exists", node_name));
        }
        let index = self.name_to_index.get(node_name).unwrap();
        let neighbors = self.graph.neighbors_directed(*index, Direction::Outgoing);
        Ok(neighbors.count() > 0)
    }

    fn is_cyclic_directed(&self) -> bool {
        petgraph::algo::is_cyclic_directed(&self.graph)
    }

    fn toposort(&self) -> Vec<String> {
        let order = petgraph::algo::toposort(&self.graph, None).unwrap();
        let mut result = vec![];

        for index in order {
            result.push(self.index_to_name.get(&index).unwrap().clone());
        }

        result
    }
}
