// Module of a graph node


use std::fmt::{Display, Debug};
use std::fmt;

// Struct of a graph node
pub struct Node<T> {
    // Node has an index, a value and may have other nodes connected to it.
    // Index of a node is NOT the same as node's position in the arena
    pub index: usize,
    pub value: T,
    // Connected nodes are accessed through their arena indexes
    // Nodes deleted from the graph are deleted from this vector as well
    // Connected nodes should be added via graph.add_node()
    connected: Vec<usize>
}

impl<T> Node<T> {
    // Constructor for a new node
    // If connected nodes are mentioned here - they are added to the node
    pub fn new(index: usize, value: T, connected: Option<Vec<usize>>) -> Self {
        if connected.is_none() {
            Node{index, value, connected: Vec::new()}
        } else {
            Node{index, value, connected: connected.unwrap()}
        }
    }

    // Getter for 'connected'
    pub fn connected(&self) -> &Vec<usize>{
        &self.connected
    }

    // Getter for mutable 'connected'
    pub fn connected_mut(&mut self) -> &mut Vec<usize> {
        &mut self.connected
    }
}

impl<T: Display + Debug> fmt::Display for Node<T> {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\nNode {}\n\tValue: {}\n\tChild nodes: {:?}", self.index, self.value, self.connected)
    }
}