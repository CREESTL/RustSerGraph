// Module of a Graph Handler

use super::graph::Graph;
use super::node::Node;
use std::fmt::Display;
use std::fs::File;
use std::io::{Write, BufReader, BufRead};

pub struct GraphHandler;


impl GraphHandler {

	// Initializer
	pub fn new() -> Self {
		GraphHandler
	}

	// Function serializes the graph into Trivial Graph Format
    pub fn serialize<T: Display>(&self, graph: &Graph<T>, path: &String) -> Result<(), String> {
        
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
                return Err(String::from("Could Not Find a Node!"));
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
                return Err(String::from("Could Not Find a Node!"));
            }
        }

        Ok(())
    }

    // Function deserializes the graph from Trivial Graph Format
    pub fn deserialize<T: Default>(&self, graph: &mut Graph<T>, path: &String) -> Result<(), String> {

        let input = File::open(path).expect("Could Not Open a File to Read From");
        let buf = BufReader::new(input);
        // Indicates if reading edges or nodes
        let mut edges = false;
        for line in buf.lines().map(|line| line.unwrap()) {
            let parts: Vec<&str> = line.split(" ").collect();
            if !edges {
                if parts.get(0).unwrap() == &"#" {
                    edges = true;
                    continue;
                }
                let index: usize = parts.get(0).unwrap().parse().unwrap();
                let label= parts[1..].join(" ");

                // WARNING!

                // Node values are NOT listed in the TGF file. That is why type of values of nodes
                // is infered from the type of the graph what will include these nodes.
                // Each node gets a default value of it;s type.
                // Only types implementing 'Default' are allowed.
                let node = Node::new(index, Default::default(), None);
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

        Ok(())
    }   

}

