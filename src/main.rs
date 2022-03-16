use graph_lib::node::Node;
use graph_lib::graph::Graph;
use graph_lib::handler::GraphHandler;


// Simple Demo

fn main() {

	// Create a simple graph with 5 nodes
	let mut graph = Graph::new();

    graph.add_node(Node::new(666,"Text", Some(vec![4])));
    graph.add_node(Node::new(4,"Text", Some(vec![3, 2])));
    graph.add_node(Node::new(3,"Text", Some(vec![777, 999])));
    graph.add_node(Node::new(2,"Text", Some(vec![8])));
    graph.add_node(Node::new(8,"Text", Some(vec![111, 222])));
    graph.add_node(Node::new(999,"Text", None));
    graph.add_node(Node::new(777,"Text", None));
    graph.add_node(Node::new(111,"Text", None));
    graph.add_node(Node::new(222,"Text", None));

	// Set the root of a graph
	graph.set_root(Some(666));

	// Paths for serializing/deserializing
	// Two different files on purpose. The second contains lots of labels
	let into_path = "./resources/write_into".to_string();
	let from_path = "./resources/read_from".to_string();

	// Create a handler that will serialize/deserialize the graph
	let handler = GraphHandler::new();
	// Write the graph into file
	handler.serialize(&mut graph, &into_path).expect("Graph Can Not be Serialized!");

	// WARNING!

	// In order for the deserialization to work correctly it is a must
	// to give a type annotation for the graph that will include the 
	// deserialized nodes!
	// Only types implementing 'Default' are available!
	let mut fresh_graph: Graph<String> = Graph::new();
	// Read graph nodes from the other file
	handler.deserialize(&mut fresh_graph, &from_path).unwrap();
	// Print the graph
	fresh_graph.print();

	

}
