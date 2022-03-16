// Module of a custom iterator
// Built-in Iterator trait doesn't fit current task. It can't mutably borrow an element of a vector if
// the vector has been borrowed already

use super::graph::Graph;
use std::fmt::Display;


pub struct GraphIter {
    // Node arena indexes are stored on the stack
    stack: Vec<usize>,
    // Vector holds the indexes of nodes that have already been visited
    // Prevents 3+ nodes loops
    visited: Vec<usize>,
}

impl GraphIter {

    // Constructor of the iterator 
    pub fn new(root: Option<usize>) -> Self {
        // If there is a root - stack starts with it
        if let Some(root) = root {
            GraphIter {
                stack: vec![root],
                visited: vec![],
            }
        // If there is no root - stack is empty
        } else {
            panic!("Please, Provide a Root Node for the Iterator!");
        }
    }

    // Function resets the iterator
    pub fn reset(&mut self, root: Option<usize>) {
        if let Some(root) = root {
            self.visited.clear();
            self.stack = vec![root];
        } else {
            panic!("Please, Provide a Root To Reset the Iterator!");
        }
    }

    // Next two functions implement a Visitor Pattern. They only borrows a graph when it's beeing called
    // Between the calls the graph can be modified in any way. A graph to borrow is passed as the second parameter.
    
    // Function returns the next item from the iterator of BREADTH-first-search
    pub fn next_breadth_search<T: Display>(&mut self, graph: &Graph<T>) -> Option<usize> {
        // remove() might panic, so we have to check for the size of the stack
        while self.stack.len() >= 1 {
            // Get the next index from the stack
            let node_index = self.stack.remove(0);

            // Only process nodes that have not been visited yet
            if self.visited.contains(&node_index) {
                continue;
            }
            self.visited.push(node_index);

            // Get the node with that index from the arena
            if let Some(node) = graph.get_node(node_index) {
                // Add it's neighbours to the stack
                for node in node.connected().iter(){
                        self.stack.push(*node);
                }

                return Some(node_index)
            } else {
                panic!("Could Not Find a Node!");
            }
        }

        return None
    }

    // Function returns the next item from the iterator of DEPTH-first-search
    pub fn next_depth_search<T: Display>(&mut self, graph: &Graph<T>) -> Option<usize> {

        // Get the next index from the stack
        while let Some(node_index) = self.stack.pop() {
            // Only process nodes that have not been visited yet
            if self.visited.contains(&node_index) {
                continue;
            }
            self.visited.push(node_index);

            // Get the node with that index from the arena
            if let Some(node) = graph.get_node(node_index) {
                // Add it's neighbours to the stack
                let mut clone = node.connected().clone();
                // Reverse the stack to process the rightmost edge first (human-readible)
                clone.reverse();
                for node in clone.iter() {
                        self.stack.push(*node);
                }

                return Some(node_index)
            } else {
                panic!("Could Not Find a Node!");
            }
        }

        return None
    }

}


