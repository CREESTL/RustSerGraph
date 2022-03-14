use graph_lib::node::Node;
use graph_lib::graph::Graph;

fn main() {
	// TODO place deserialization from file here

	// Create a simple graph with 5 nodes
	let mut graph = Graph::new();

	graph.add_node(Node::new(666,"Text"));
	graph.add_node(Node::new(4,"Text"));
	graph.add_node(Node::new(3,"Text"));
	graph.add_node(Node::new(2,"Text"));
	graph.add_node(Node::new(777,"Text"));
	graph.add_node(Node::new(999,"Text"));
	graph.add_node(Node::new(8,"Text"));
	graph.add_node(Node::new(111,"Text"));
	graph.add_node(Node::new(222,"Text"));


	// Set the root of a graph
	graph.set_root(Some(666));

	// Add edges
	graph.add_edge(666, 4);
	graph.add_edge(4, 3);
	graph.add_edge(4, 2);
	graph.add_edge(3, 777);
	graph.add_edge(3, 999);
	graph.add_edge(2, 8);
	graph.add_edge(8, 111);
	graph.add_edge(8, 222);


	// Tests
	//graph.add_edge(666, 1);
	//graph.add_edge(666, 1);
	//graph.add_edge(1,666);
	//graph.add_edge(1,1);


	graph.print();

	let into_path = "/home/creestl/programming/blockchain/pixel_plex/ser_graph/write_into".to_string();
	let from_path = "/home/creestl/programming/blockchain/pixel_plex/ser_graph/write_from".to_string();
	graph.serialize(&into_path).expect("Graph Can Not Be Serialized!");
	//graph.deserialize(&from_path);

}