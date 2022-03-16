// Module of a Graph Handler

use super::graph::Graph;
use super::node::Node;
use std::fmt::Display;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};

pub struct GraphHandler;

impl GraphHandler {

	// Function adds the provided instance of a node into the graph
	pub fn add_to_graph<T>(graph: &mut Graph<T>, node: Node<T>,) 
		where T: Default + Display
	{
		graph.add_node(node);
	}

	// Function serializes the graph into Trivial Graph Format
    pub fn serialize<T>(graph: &Graph<T>, path: &String) -> Result<(), Error> 
    	where T: Default + Display
    {
        
        let mut iter = graph.iterator();            
        let mut output = File::create(path).expect("Could Not Create a File to Write Into");
        // Iterate over all nodes and write each node data into the file
        while let Some(i) = iter.next_breadth_search(graph) {
            if let Some(node) = graph.get_node(i) {
                writeln!(output, "{} {}", node.index, node.value).expect("Could Not Write a Node to File!");
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
}

