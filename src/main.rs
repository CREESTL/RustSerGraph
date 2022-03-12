mod lib;
use lib::{Node, Tree};

fn main() {
	// TODO place deserialization from file here

	// Create a simple tree with 5 nodes
	let mut tree = Tree::new();

	tree.add_node(Node::new(666,"Text"));
	tree.add_node(Node::new(4,"Text"));
	tree.add_node(Node::new(3,"Text"));
	tree.add_node(Node::new(2,"Text"));
	tree.add_node(Node::new(777,"Text"));
	tree.add_node(Node::new(999,"Text"));
	tree.add_node(Node::new(8,"Text"));
	tree.add_node(Node::new(111,"Text"));
	tree.add_node(Node::new(222,"Text"));


	// Set the root of a tree
	tree.set_root(Some(666));

	// Add edges
	tree.add_edge(666, 4);
	tree.add_edge(4, 3);
	tree.add_edge(4, 2);
	tree.add_edge(3, 777);
	tree.add_edge(3, 999);
	tree.add_edge(2, 8);
	tree.add_edge(8, 111);
	tree.add_edge(8, 222);


	// Tests
	//tree.add_edge(666, 1);
	//tree.add_edge(666, 1);
	//tree.add_edge(1,666);
	//tree.add_edge(1,1);


	tree.print();

}