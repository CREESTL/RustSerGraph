pub mod node;
pub mod graph;
pub mod iterator;
pub mod handler;


#[cfg(test)]
mod tests {

    use super::{node::Node, graph::Graph, iterator::GraphIter};

    // Tests for Node

    #[test]
    pub fn create_connected_node() {
        let node = Node::new(888, 1, Some(vec![777, 888, 111]));
        assert!(!node.connected().is_empty());
    }

    #[test]
    pub fn create_unconnected_node() {
    	let node = Node::new(99, -10000, None);
    	assert_eq!(node.connected(), &Vec::new());
    }

    #[test]
    pub fn correct_node_print() {
        let node = Node::new(88, "Richard Mille", None);
        assert_eq!(format!("{}", node), "\nNode 88\n\tValue: Richard Mille\n\tChild nodes: []");
    }


    // Tests for Graph

    #[test]
    pub fn create_graph_no_root() -> Result<(), String> {
    	let graph = Graph::<&str>::new();
    	if let Some(_) = graph.get_node(0) {
    		Err(String::from("In New Grapg There Must Be No Root!"))
    	} else {
    		Ok(())
    	}
    }

    #[test]
    #[should_panic]
    pub fn try_add_node_twice() {
        let mut graph = Graph::<&str>::new();
        graph.add_node(Node::new(666,"Text", None));
        graph.add_node(Node::new(666,"Text", None));
    }


    #[test]
    #[should_panic]
    pub fn remove_unexisting_node() {
        let mut graph = Graph::<&str>::new();
        graph.remove_node(333);
    }


    #[test]
    pub fn try_get_unexisting_node() -> Result<(), String> {
        let graph = Graph::<&str>::new();
        if graph.get_node(1).is_some(){
            return Err(String::from("Successfully accessed an unexisting node!"))
        } else {
            Ok(())
        }
    }

    #[test]
    pub fn try_find_unexisting_node() {
        let graph = Graph::<&str>::new();
        assert!(!graph.in_graph(111));

    }


    #[test]
    #[should_panic]
    pub fn try_set_root_none() {
        let node = Node::new(666,"Text", None);
        let mut graph = Graph::<&str>::new();
        graph.add_node(node);
        // Also try to change the root
        graph.set_root(Some(666));
        graph.set_root(Some(777));
        graph.set_root(None);

    }


    #[test]
    #[should_panic]
    pub fn try_add_same_edge() {
        let node1 = Node::new(666,"Text", None);
        let node2 = Node::new(777,"Daniel", None);
        let mut graph = Graph::<&str>::new();
        graph.add_node(node1);
        graph.add_node(node2);
        graph.add_edge(666, 777);
        graph.add_edge(666, 777);
    }

    #[test]
    #[should_panic]
    pub fn try_add_edge_unexisting_nodes() {
        let mut graph = Graph::<&str>::new();
        graph.add_edge(666, 777);
    }

    #[test]
    #[should_panic]
    pub fn try_add_forbidden_loop() {
        let node1 = Node::new(666,"Text", None);
        let mut graph = Graph::<&str>::new();
        graph.add_node(node1);
        graph.add_edge(666, 666);
    }


    // Tests for Iterator


    #[test]
    #[should_panic]
    pub fn try_create_iterator_no_root() {
        let iterator = GraphIter::new(None);
    }

    #[test]
    #[should_panic]
    pub fn try_reset_iterator_no_root() {
        let mut iterator = GraphIter::new(Some(1));
        iterator.reset(None);
    }

    #[test]
    pub fn check_bfs() -> Result<(), String>{
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

        let correct_order = vec![666, 4, 3, 2, 777, 999, 8, 111, 222];

        let mut iterator = GraphIter::new(Some(666));

        let mut i = 0;
        let mut pass = true;
        while let Some(index) = iterator.next_breadth_search(&graph) {
            if correct_order[i] != index {
                pass = false;
            }
            i += 1;
        }

        match pass {
            true => return Ok(()),
            false => return Err(String::from("Wrong Next Element of BFS!"))
        }


    }

    #[test]
    pub fn check_dfs() -> Result<(), String>{
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

        let correct_order = vec![666, 4, 3, 777, 999, 2, 8, 111, 222];

        let mut iterator = GraphIter::new(Some(666));

        let mut i = 0;
        let mut pass = true;
        while let Some(index) = iterator.next_depth_search(&graph) {
            if correct_order[i] != index {
                pass = false;
            }
            i += 1;
        }

        match pass {
            true => return Ok(()),
            false => return Err(String::from("Wrong Next Element of BFS!"))
        }
    }

    #[test]
    pub fn check_iterating_loops() -> Result<(), String>{
        let mut graph = Graph::new();

        graph.add_node(Node::new(666,"Text", None));
        graph.add_node(Node::new(4,"Text", None));
        graph.add_node(Node::new(3,"Text", None));


        graph.add_edge(666, 4);
        graph.add_edge(4, 3);
        graph.add_edge(3, 666);

        let mut iterator = GraphIter::new(Some(666));

        let mut last = 0;
        while let Some(index) = iterator.next_breadth_search(&graph) {
            last = index;
        }

        match last {
            3 => return Ok(()),
            _ => return Err(String::from("Wrong Next Element of BFS!"))
        }


    }


}

