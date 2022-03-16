use graph_lib::node::Node;
use graph_lib::graph::Graph;
use graph_lib::handler::GraphHandler;

fn main() {
	// TODO place deserialization from file here

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

	graph.print();

	let into_path = "/home/creestl/programming/blockchain/pixel_plex/ser_graph/write_into".to_string();
	let from_path = "/home/creestl/programming/blockchain/pixel_plex/ser_graph/write_from".to_string();

	let handler = GraphHandler::new();
	handler.serialize(&mut graph, &into_path).expect("Graph Can Not Be Serialized!");
	match handler.deserialize(&from_path) {
		Err(String) => println!("Could not parse the file!"),
		_ => ()
	}

}
