mod lib;
use lib::{Node, Tree};

fn main() {
	// TODO place deserialization from file here

	// Create a simple tree with 5 nodes
	let mut tree = Tree::new();

	tree.add_node(Node::new(1,"Simple"));
	tree.add_node(Node::new(2,"Text"));
	tree.add_node(Node::new(3,"Generic"));
	tree.add_node(Node::new(4,"Is"));
	tree.add_node(Node::new(666,"Working"));

	// Set the root of a tree
	tree.set_root(Some(666));

	// Add edges
	tree.add_edge(666, 4);
	tree.add_edge(4, 3);
	tree.add_edge(4, 2);
	tree.add_edge(3, 1);
	tree.add_edge(2, 1);


	// Tests
	//tree.add_edge(666, 1);
	//tree.add_edge(666, 1);
	//tree.add_edge(1,666);
	//tree.add_edge(1,1);


	tree.print();

}