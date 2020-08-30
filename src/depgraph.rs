use petgraph::prelude::*;
use std::collections::HashMap;

pub struct GraphData {
    // TODO: Graph
    pub graph: DiGraphMap<i32, i8>,
    pub index_to_name: HashMap<i32, String>,
    pub name_to_index: HashMap<String, i32>,
}

pub fn init_dep_graph() -> GraphData {
    GraphData {
        graph: Default::default(),
        index_to_name: Default::default(),
        name_to_index: Default::default(),
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
        let index = self.name_to_index.get(node_name_a).unwrap();
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
