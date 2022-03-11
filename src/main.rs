mod lib;
use lib::{Node, Tree};

fn main() {
	// TODO place deserialization from file here

	// Create a simple tree with 5 nodes
	let mut tree = Tree::new();
	tree.add_node(Node::new(1,"Simple", vec![]));
	tree.add_node(Node::new(2,"Text", vec![1]));
	tree.add_node(Node::new(3,"Generic", vec![1]));
	tree.add_node(Node::new(4,"Is", vec![3, 2]));
	tree.add_node(Node::new(666,"Working", vec![4]));

	// Set the root of a tree
	tree.set_root(Some(666));

	// Tests
	//tree.add_edge(666, 1);
	//tree.add_edge(666, 1);
	//tree.add_edge(1,666);
	// tree.add_edge(1,1);


	tree.print();

}