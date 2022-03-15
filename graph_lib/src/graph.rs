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
    pub fn iterator(&mut self) -> GraphIter {
        GraphIter::new(self.root)
    }

    // Function serializes the graph into Trivial Graph Format
    pub fn serialize(&mut self, path: &String) -> Result<(), Error> {
        
        let mut iter = self.iterator();            
        let mut output = File::create(path).expect("Could Not Create a File to Write Into");
        // Iterate over all nodes and write each node data into the file
        while let Some(i) = iter.next_breadth_search(&self) {
            if let Some(node) = self.get_node(i) {
                writeln!(output, "{} {}", node.index, node.value).expect("Could Not Write a Node to File!");
            } else {
                panic!("Could Not Find a Node!");
            }
        }
        // Separator between strings of nodes and strings of edges
        writeln!(output, "#").expect("Could Not Write a Separator to File!");
        // Reset the iterator to start iterating again
        iter.reset(self.root);

        // Iterate over all nodes and write each pair of connected nodes
        while let Some(i) = iter.next_breadth_search(&self) {
            if let Some(node) = self.get_node(i) {
                for another in node.connected().iter() {
                    // No labels for edges are written into the file
                    writeln!(output, "{} {}", node.index, another).expect("Could Not Write an Edge to File!");
                }
            } else {
                panic!("Could Not Find a Node!");
            }
        }

        Ok(())
    }

    // Function deserializes the graph from Trivial Graph Format
    // pub fn deserialize(&mut self, path: &String) -> Result<(), Error> {
    //     let input = File::open(path).expect("Could Not Open a File to Read From");
    //     let buf = BufReader::new(input);
    //     // Indicates if reading edges or nodes
    //     let mut edges = false;
    //     for line in buf.lines().map(|line| line.unwrap()) {
    //         let parts: Vec<&str> = line.split(" ").collect();
    //         //println!("parts are {:?}", parts);
    //         if !edges {
    //             if parts.get(0).unwrap() == &"#" {
    //                 edges = true;
    //                 continue;
    //             }
    //             let index: usize = parts.get(0).unwrap().parse().unwrap();
    //             let label= parts[1..].join(" ");
    //             let node : T= Node::new(index, label);
    //             self.add_node(node);
    //             println!("Node Index Is {}", index);
    //         } else {
    //             let first = parts.get(0).unwrap();
    //             let second = parts.get(1).unwrap();
    //             println!("Firt Index Is {}, Second Index Is {}", first, second);
    //         }
    //     }

    //     Ok(())
    // }   

    // Function prints the graph
    pub fn print(&mut self) {
        
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

