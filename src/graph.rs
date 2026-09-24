use std::fmt;
use std::collections::{HashMap, HashSet};
use crate::parser::Rule;

pub struct Node {
    adjacent_nodes: HashSet<u32>,
    visited: bool
}

pub struct Graph {
    nodes: HashMap<u32, Node>,
    temp: Vec<u32>,
    temp_set: HashSet<u32>
}

impl Node {
    pub fn new() -> Self {
        Node {
            adjacent_nodes: HashSet::new(),
            visited: false
        }
    }
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut edges: Vec<&u32> = self.adjacent_nodes.iter().collect();
        edges.sort();

        let edge_strings: Vec<String> = edges.iter().map(|id| format!("R{}", id)).collect();
        let joined_edges = edge_strings.join(",");

        write!(f, "{}", joined_edges)
    }
}

impl Graph {
    pub fn new(rules: Vec<Rule>) -> Self {
        let mut node_map = HashMap::new();
        for i in 0..rules.len() {
            node_map.insert(i.try_into().unwrap(), Node::new());
        }
        
        Graph {
            nodes: node_map,
            temp: Vec::new(), 
            temp_set: HashSet::new()
        }
    }

    pub fn add_edge(&mut self, from_id: u32, to_id: u32) {
        self.nodes.entry(from_id).or_insert_with(|| Node::new()).adjacent_nodes.insert(to_id);
    }

    pub fn dfs_forest(&mut self) -> Vec<u32> {
        for node in self.nodes.values_mut() {
            node.visited = false;
        }

        let mut keys: Vec<u32> = self.nodes.keys().cloned().collect();
        keys.sort();

        for key in keys {
            if let Some(node) = self.nodes.get(&key) {
                if !node.visited {
                    self.dfs(key);
                }
            }
        }
        self.temp.clone()
    }

    pub fn dfs_forest_og(&mut self, mut eval_order: Vec<u32>) -> Vec<HashSet<u32>> {
        for node in self.nodes.values_mut() {
            node.visited = false;
        }
        let mut sccs: Vec<HashSet<u32>> = Vec::new();
        while !eval_order.is_empty() {
            let top_node_id = eval_order.pop().unwrap();
            if let Some(node) = self.nodes.get(&top_node_id) {
                if !node.visited {
                    self.dfs_collect(top_node_id);
                    sccs.push(self.temp_set.clone());
                    self.temp_set.clear();
                }
            }
        }
        sccs
    }

    pub fn dfs_collect(&mut self, rule_num: u32) {
        if let Some(node) = self.nodes.get_mut(&rule_num) {
            if node.visited {
                return;
            }
            node.visited = true;
        } else {
            return;
        }
        self.temp_set.insert(rule_num);

        let mut neighbors: Vec<u32> = match self.nodes.get(&rule_num) {
            Some(node) => node.adjacent_nodes.iter().cloned().collect(),
            None => return,
        };
        neighbors.sort();

        for item in neighbors {
            if let Some(neighbor_node) = self.nodes.get(&item) {
                if !neighbor_node.visited {
                    self.dfs_collect(item);
                }
            }
        }
    }

    pub fn dfs(&mut self, rule_num: u32) {
        if let Some(node) = self.nodes.get_mut(&rule_num) {
            if node.visited {
                return;
            }
            node.visited = true;
        } else {
            return;
        }

        let mut neighbors: Vec<u32> = match self.nodes.get(&rule_num) {
            Some(node) => node.adjacent_nodes.iter().cloned().collect(),
            None => return,
        };
        neighbors.sort();

        for item in neighbors {
            if let Some(neighbor_node) = self.nodes.get(&item) {
                if !neighbor_node.visited {
                    self.dfs(item);
                }
            }
        }
        self.temp.push(rule_num);
    }

    pub fn dependent(&mut self, node_id: u32) -> bool {
        let printable: HashSet<u32> = self.nodes.get(&node_id).unwrap().adjacent_nodes.clone();
        for integer in printable {
            if integer == node_id {
                return true;
            }
        }
        return false;
    }
}

impl fmt::Display for Graph {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut sorted_nodes: Vec<(&u32, &Node)> = self.nodes.iter().collect();
        sorted_nodes.sort_by_key(|&(id, _)| id);

        for (node_id, node) in sorted_nodes {
            writeln!(f, "R{}:{}", node_id, node)?;
        }

        Ok(())
    }
}
