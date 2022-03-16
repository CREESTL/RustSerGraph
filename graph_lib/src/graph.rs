// Module of a graph

use super::node::Node;
use super::iterator::GraphIter;
use std::fmt::Display;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};


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

        // Check if such node is not present in the graph
        if !self.get_node(node.index).is_some() {
            self.arena.push(node);
        } else {
            panic!("Node {} Is Already in The Graph", node.index);
        }


    }

    // Function removes a node with a given arena index
    pub fn remove_node(&mut self, index: usize) {
        if !self.in_graph(index) {
            panic!("Node {} does not exist in the graph!", index);
        }
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
        // Once root has been set to 'Some' it can't be set to 'None'
        if self.root.is_some() && root.is_none() {
            panic!("Can't Set the Root to 'None' If It Has Already Been Set To 'Some'");
        }
        // Check if a given root exists in graph
        if root.is_some() && self.in_graph(root.unwrap()) {
            self.root = root;
        }
    }

    // Function checks if node exists in the graph
    pub fn in_graph(&self, index: usize) -> bool {
        if self.get_node(index).is_some() {
            return true;
        }
        return false;
    }

    // Function creates a directed edge of the graph between two nodes
    pub fn add_edge(&mut self, from: usize, to: usize) {
        // Check if both nodes are in the graph
        if self.in_graph(from) && self.in_graph(to) {
            // Start and end of the edge must be different nodes
            // That is the only forbidden case of a loop
            if to == from {
                panic!("Can't Form an Edge from the Node to Itself!");
            } else {

                // Check if edge does not exist yet
                // Multiple edges from one node to another are forbidden
                if !self.get_node(from).unwrap().connected().contains(&to) {
                    self.get_node_mut(from).unwrap().connected_mut().push(to);
                } else {
                    panic!("Multiple Edges From Node {} to Node {} are Forbidden!", to, from);
                }
            }
        } else {
            panic!("Both Nodes Must Be First Added To The Graph!");
        }
    }

    // Function returns a custom iterator over the graph
    pub fn iterator(&self) -> GraphIter {
        GraphIter::new(self.root)
    }

   
    // Function prints the graph
    pub fn print(&self) {
        
        println!("\nRoot Node: {}", self.root.unwrap());

        // Create an iterator of a graph
        let mut graph_iter = self.iterator();
        // Iterate over the graph and print it's nodes
        while let Some(i) = graph_iter.next_breadth_search(&self) {
            if let Some(node) = self.get_node(i){
                println!("{}", node);
            } else {
                panic!("Could Not Find a Node!");
            }
        }          
    }
}

