#![allow(dead_code)]

// Export structs
pub use node::Node;
pub use graph::Graph;
pub use iter::GraphIter;


// Module of a graph node
mod node {

    use std::fmt::Display;
    use std::fmt;

    // Struct of a graph node]
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

}

// Module of a graph
mod graph {
    use super::node::Node;
    use super::iter::GraphIter;
    use std::fmt::Display;
    use std::fs::File;
    use std::io::{Write, BufReader, BufRead, Error};
    use std::slice::SliceIndex;


    // Struct of a graph
    pub struct Graph<T> {
        // Graph has a root and an arena
        // Arena is a vector holding nodes of a graph. Allows for random access without nested borrowing
        // Access to each node from arena is through it's index.
        pub arena: Vec<Node<T>>,
        // Root is one of the nodes in arena. Access through index as well.
        pub root: Option<usize>,
    }

    impl<T: Display> Graph<T>{
        // Constructor of a graph
        // At first, graph has no root. It must be set with set_root()
        pub fn new() -> Self {
            Graph{arena: Vec::new(), root: None}
        }

        // Function adds a node to the graph
        pub fn add_node(&mut self, node: Node<T>) {  

            for el in self.arena.iter() {
                // Check if any previous node connects to the current node
                if el.connected.contains(&node.index) {
                    // Check if current node connects to that previous one
                    if node.connected.contains(&el.index) {
                        panic!("Graph Loops Are Forbidden!");
                    }
                }
            }

            self.arena.push(node);
        }

        // Function removes a node with a given arena index
        pub fn remove_node(&mut self, index: usize) {
            self.arena.retain(|x| x.index != index);
        }

        // Function gets the node from the graph (borrows it)
        pub fn get_node(&self, index: usize) -> Option<&Node<T>> {
            for node in self.arena.iter() {
                if node.index == index {
                    return Some(node)
                }
            }
            None
        }

        // Function gets a mutable node from the graph (mutably borrows it)
        pub fn get_node_mut(&mut self, index: usize) -> Option<&mut Node<T>> {
            for node in self.arena.iter_mut() {
                if node.index == index {
                    return Some(node)
                }
            }
            None
        }


        // Function makes a node with a given arena index a root of a graph
        pub fn set_root(&mut self, root: Option<usize>) {
            self.root = root;
        }

        // Function checks if node exists in the graph
        pub fn in_graph(&self, index: usize) -> bool {
            if let Some(_) = self.get_node(index) {
                return true;
            }
            return false;
        }

        // Function creates a directed edge of the graph between two nodes
        pub fn add_edge(&mut self, from: usize, to: usize) {
            // Check if both nodes are in the graph
            if self.in_graph(from) && self.in_graph(to) {
                // Start and end of the edge must be different nodes
                if to == from {
                    panic!("Graph Loops are Forbidden");
                } else {
                    // Check if edge does not exist yet
                    if !self.get_node(from).unwrap().connected.contains(&to) {
                        // Check if a parent node already has edge to this child. It that case creating
                        // another edge makes a loop
                        if !self.get_node(to).unwrap().connected.contains(&from) {
                            self.get_node_mut(from).unwrap().connected.push(to);
                        } else {
                            panic!("Graph Loops are Forbidden");
                        }
                    } else {
                        panic!("Node {} Is Already a Child for Node {}", to, from);
                    }
                }
            } else {
                panic!("Both Nodes Must Be First Added To The Graph!");
            }
        }

        // Function returns a custom iterator over the graph
        pub fn iterator(&mut self) -> GraphIter {
            GraphIter::new(self.root)
        }

        // Function serializes the graph into Trivial Graph Format
        pub fn serialize(&mut self, path: String) -> Result<(), Error>{
            
            let mut iter = self.iterator();            
            let mut output = File::create(path)?;
            while let Some(i) = iter.next(&self) {
                if let Some(node) = self.get_node(i) {
                    writeln!(output, "{} {}", node.index, node.value).expect("Could Not Write a Node to File!");
                }
            }
            writeln!(output, "#").expect("Could Not Write a Separator to File!");
            // Reset the iterator to start iterating agaim
            iter.reset(self.root);
            while let Some(i) = iter.next(&self) {
                if let Some(node) = self.get_node(i) {
                    for another in node.connected.iter() {
                        // No labels for edges are written into the file
                        writeln!(output, "{} {}", node.index, another).expect("Could Not Write an Edge to File!");
                    }
                }
            }



            Ok(())

        }


    }


    // Separate implementation for Display bound
    impl<T: Display> Graph<T>{
        // Function prints the graph
        pub fn print(&mut self) {
            
            println!("\nRoot Node: {}", self.root.unwrap());

            // Create an iterator of a graph
            let mut graph_iter = self.iterator();
            // Iterate over the graph and print it's nodes
            while let Some(i) = graph_iter.next(&self) {
                if let Some(node) = self.get_node(i){
                    println!("{}", node);   
                }
            }          
        }
    }


}


// Module of a custom iterator
// Built-in Iterator trait doesn't fit current task. It can't mutably borrow an element of a vector if
// the vector has been borrowed already
mod iter{

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


}

