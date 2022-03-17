use super::node::Node;
use super::iterator::GraphIter;
use std::fmt::{Display, Debug};

// Module of a graph

// Struct of a graph
pub struct Graph<T> {
    // Graph has a root and an arena
    // Arena is a vector holding nodes of a graph. Allows for random access without nested borrowing
    // Access to each node from arena is through it's index.
    pub arena: Vec<Node<T>>,
    // Root is one of the nodes in arena. Access through index as well.
    pub root: Option<usize>,
}

impl<T> Graph<T> {
    // Constructor of a graph
    // At first, graph has no root. It must be set with set_root()
    pub fn new() -> Self {
        Graph{arena: Vec::new(), root: None}
    }

    // Function adds a node to the graph
    pub fn add_node(&mut self, node: Node<T>) -> Result<(), String>{  

        // Check if such node is not present in the graph
        if !self.get_node(node.index).is_some() {
            self.arena.push(node);
            Ok(())
        } else {
            Err(format!("Node {} is Already in The Graph", node.index))
        }
    }


    // Function checks if node exists in the graph
    pub fn in_graph(&self, index: usize) -> bool {
        if self.get_node(index).is_some() {
            return true;
        }
        return false;
    }

    // Function removes a node with a given index
    pub fn remove_node(&mut self, index: usize) -> Result<(), String> {
        if !self.in_graph(index) {
            return Err(format!("Node {} does not Exist in the Graph!", index))
        }
        self.arena.retain(|x| x.index != index);
        Ok(())
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


    // Function makes a node with a given index a root of a graph
    pub fn set_root(&mut self, root: Option<usize>) -> Result<(), String> {
        // Once root has been set to 'Some' it can't be set to 'None'
        if self.root.is_some() && root.is_none() {
            return Err(String::from("Can't Set the Root to 'None' if It Has Already Been Set to 'Some'"))
        }
        // Check if a given root exists in graph
        if root.is_some() && self.in_graph(root.unwrap()) {
            self.root = root;
            Ok(())
        } else {
            return Err(format!("Node {} is not in the Graph. Can't Set It to Root", root.unwrap()))
        }
    }


    // Function creates a directed edge of the graph between two nodes
    pub fn add_edge(&mut self, from: usize, to: usize) -> Result<(), String> {
        // Check if both nodes are in the graph
        if self.in_graph(from) && self.in_graph(to) {
            // Start and end of the edge must be different nodes
            // That is the only forbidden case of a loop
            if to == from {
                return Err(String::from("Can't Form an Edge From the Node to Itself!"))
            } else {
                // Check if edge does not exist yet
                // Multiple edges from one node to another are forbidden
                if !self.get_node(from).unwrap().connected().contains(&to) {
                    self.get_node_mut(from).unwrap().connected_mut().push(to);
                    Ok(())
                } else {
                    return Err(format!("Multiple Edges From Node {} to Node {} are Forbidden!", to, from))
                }
            }
        } else {
            return Err(String::from("Both Nodes Must Be First Added To The Graph!"))
        }
    }

    // Function deletes an edge between two nodes
    pub fn remove_edge(&mut self, from: usize, to: usize) -> Result<(), String> {
        // Check if both nodes are in the graph
        if self.in_graph(from) && self.in_graph(to) {
            if let Some(node) = self.get_node_mut(from) {
                // Check if the edge exists
                if node.connected().contains(&to) {
                    node.connected_mut().retain(|el| *el != to);
                    return Ok(())
                } else {
                    return Err(String::from("The Edge Between Given Nodes Does Not Exist!"))
                }
            } else {
                return Err(String::from("Could Not Find the First Given Node in the Graph"))
            }
        } else {
            Err(String::from("Both Nodes Must Be First Added To The Graph!"))
        }        
    }

    // Function returns a custom iterator over the graph
    pub fn iterator(&self) -> GraphIter {
        GraphIter::new(self.root).unwrap()
    }

}


// Implementation of traits for propper output
impl<T: Display + Debug> Graph<T> {    
    // Function prints the graph
    pub fn print(&self) {
        println!("\nRoot Node: {}", self.root.unwrap());
        // Create an iterator of a graph
        let mut graph_iter = self.iterator();
        // Iterate over the graph and print each node
        while let Some(i) = graph_iter.next_breadth_search(&self) {
            if let Some(node) = self.get_node(i) {
                println!("{}", node);
            } else {
                panic!("Could Not Find a Node!");
            }
        }          
    }
}

