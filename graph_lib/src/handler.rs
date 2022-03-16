// Module of a Graph Handler

use super::graph::Graph;
use super::node::Node;
use std::fmt::Display;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

pub struct GraphHandler;

impl GraphHandler {

	// Initializer
	pub fn new() -> Self {
		GraphHandler
	}

	// Function serializes the graph into Trivial Graph Format
    pub fn serialize<T: Display>(&self, graph: &Graph<T>, path: &String) -> Result<(), Error> {
        
        let mut iter = graph.iterator();            
        let mut output = File::create(path).expect("Could Not Create a File to Write Into");
        // Iterate over all nodes and write each node data into the file
        while let Some(i) = iter.next_breadth_search(graph) {
            if let Some(node) = graph.get_node(i) {
            	// Mark the root of the graph in TGF with "Root"
            	if graph.root.unwrap() == node.index {
                	writeln!(output, "{} Root", node.index).expect("Could Not Write a Node to File!");
            	} else {
                	writeln!(output, "{}", node.index).expect("Could Not Write a Node to File!");
            	}
            } else {
                panic!("Could Not Find a Node!");
            }
        }
        // Separator between strings of nodes and strings of edges
        writeln!(output, "#").expect("Could Not Write a Separator to File!");
        
        // Reset the iterator to start iterating again
        iter.reset(graph.root);

        // Iterate over all nodes and write each pair of connected nodes
        while let Some(i) = iter.next_breadth_search(graph) {
            if let Some(node) = graph.get_node(i) {
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
    pub fn deserialize(&self, path: &String) -> Result<(), Error> {

        let input = File::open(path).expect("Could Not Open a File to Read From");
        let buf = BufReader::new(input);
        // Indicates if reading edges or nodes
        let mut edges = false;
        // Create a new graph
        let mut graph = Graph::new();
        for line in buf.lines().map(|line| line.unwrap()) {
            let parts: Vec<&str> = line.split(" ").collect();
            //println!("parts are {:?}", parts);
            if !edges {
                if parts.get(0).unwrap() == &"#" {
                    edges = true;
                    continue;
                }
                let index: usize = parts.get(0).unwrap().parse().unwrap();
                let label= parts[1..].join(" ");
                let node = Node::new(index, "EMPTY", None);
                graph.add_node(node);
                // One of the nodes must be the root
                if label == String::from("Root") {
                	graph.set_root(Some(index));
                }
            } else {
                let from = parts.get(0).unwrap().parse().unwrap();
                let to = parts.get(1).unwrap().parse().unwrap();
                graph.add_edge(from, to);
            }
        }

        println!("\n\n\nGRAPH DESER!");
        graph.print();

        Ok(())
    }   

    // WORKS!
    // pub fn deserialize(&self, i: i32) {
    // 	let mut graph = Graph::new();
    // 	let node = Node::new(1, 2, None);
    // 	self.add_to_graph(&mut graph, node);
    // }
}

