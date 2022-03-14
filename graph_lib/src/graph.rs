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
    pub fn serialize(&mut self, path: &String) -> Result<(), Error> {
        
        let mut iter = self.iterator();            
        let mut output = File::create(path).expect("Could Not Create a File to Write Into");
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
        while let Some(i) = graph_iter.next(&self) {
            if let Some(node) = self.get_node(i){
                println!("{}", node);
            }
        }          
    }
}

