mod lib;
use lib::{Node, Tree};

fn main() {
	// Create a simple tree with 5 nodes
	let mut tree = Tree::new();
	let a = tree.add_node(Node::new("Simple", None, None));
	let b = tree.add_node(Node::new("Text", None, None));
	let c = tree.add_node(Node::new("Generic", Some(a), Some(b)));
	let d = tree.add_node(Node::new("Is", None, None));
	let e = tree.add_node(Node::new("Working", Some(c), Some(d)));

	// Set the root of a tree
	tree.set_root(Some(e));

	// Create an iterator of a tree
	let mut tree_iter = tree.iter();
	// Iterate over the tree and print it's nodes
	while let Some(i) = tree_iter.next(&tree) {
		let node = tree.get_node(i).expect("No suck node!");
		println!("{}", node.value);
	}
}