// Module of a custom iterator
// Built-in Iterator trait doesn't fit current task. It can't mutably borrow an element of a vector if
// the vector has been borrowed already

use super::graph::Graph;
use std::fmt::Display;


pub struct GraphIter{
    // Node arena indexes are stored on the stack
    stack: Vec<usize>
}

impl GraphIter{

    // Constructor of the iterator 
    pub fn new(root: Option<usize>) -> Self {
        // If there is a root - stack starts with it
        if let Some(root) = root {
            GraphIter {
                stack: vec![root]
            }
        // If there is no root - stack is empty
        } else {
            panic!("Please, Provide a Root Node for the Iterator!");
        }
    }

    // Function resets the iterator
    pub fn reset(&mut self, root: Option<usize>) {
        if let Some(root) = root {
            self.stack = vec![root];
        } else {
            self.stack = vec![];
        }
    }

    // Function returns the next item from the iterator of breadth-first-search
    // This function implements a Visitor Pattern. It only borrows a graph when it's beeing called
    // Between the calls the graph can be modified in any way. A graph to borrow is passed as the second parameter
    pub fn next<T: Display>(&mut self, graph: &Graph<T>) -> Option<usize> {
        let mut c = self.stack.clone();
        c.reverse();
        // Get the next index from the stack
        while let Some(node_index) = self.stack.pop(){
            // Get the node with that index from the arena
            if let Some(node) = graph.get_node(node_index) {
                // Add it's neighbours to the stack
                let mut clone = node.connected.clone();
                // Reverse the stack to process the rightmost edge first (human-readible)
                clone.reverse();
                for node in node.connected.iter(){
                        self.stack.push(*node);
                }

                return Some(node_index)
            }
        }

        return None
    }

}

