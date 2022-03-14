// Module of a graph node


use std::fmt::Display;
use std::fmt;

// Struct of a graph node
pub struct Node<T> {
    // Node has an index, a value and may have other nodes connected to it.
    // Index of a node is NOT the same as node's position in the arena
    pub index: usize,
    pub value: T,
    // Connected nodes are accessed through their arena indexes
    // Nodes deleted from the graph are deleted from this vector as well
    pub connected: Vec<usize>
}

impl<T> Node<T> {
    // Constructor for a new node
    pub fn new(index: usize, value: T) -> Self {
        Node{index, value, connected: Vec::new()}
    }
}

impl<T: Display> fmt::Display for Node<T> {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "\nNode {}\n\tValue: {}\n\tChild nodes: {:?}", self.index, self.value, self.connected)
    }
}



