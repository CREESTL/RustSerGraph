use graph_lib::node::Node;
use graph_lib::graph::Graph;
use graph_lib::handler::GraphHandler;


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

	graph.print();

	let into_path = "./write_into".to_string();
	let from_path = "./write_from".to_string();


	let handler = GraphHandler::new();
	handler.serialize(&mut graph, &into_path).expect("Graph Can Not Be Serialized!");

	// WARNING!

	// In order for the deserialization to work correctly it is a must
	// to give a type annotation for the graph that will include the 
	// deserialized nodes!
	// Only types implementing 'Default' are available!
	let mut fresh_graph: Graph<String> = Graph::new();
	handler.deserialize(&mut fresh_graph, &from_path).unwrap();
	fresh_graph.print();

	

}
