fn main() {}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::fmt::{Display, Formatter};
    use std::ops::Add;
    use std::rc::Rc;


    #[test]
    fn box_new() {
        let n = Box::new(5); // create a new int value on heap
        let m = n.add(3); // n is not int, but introduce arithmetic methods

        assert_eq!(m, 8);
    }

    #[test]
    fn box_derefer() {
        let n = Box::new(5);

        assert_eq!(*n, 5);
    }

    #[test]
    fn rc_basic_usage() {
        let s1 = Rc::new(String::from("Presenting Rc pointer"));
        println!("{:?}", Rc::strong_count(&s1));

        let s2 = s1.clone();  // clone increase only the counter in Rc
                                        // there is no deep copy

        println!("{:?}, {:?}", s1, s2); // one can use both s1 & s2

        println!("{:?}", Rc::strong_count(&s1)); // counter is increased

    }

    #[test]
    #[allow(unused)]
    fn rc_create_graph_intro() {
        #[derive(Debug)]
        struct Node {
            label: String,
        }

        impl Node {
            fn new(label: String) -> Rc<Node> {
                Rc::new(Node { label })
            }
        }

        #[derive(Debug)]
        struct Edge {
            nodes: (Rc<Node>, Rc<Node>),
        }

        impl Edge {
            fn new(node1: Rc<Node>, node2: Rc<Node>) -> Rc<Edge> {
                let edge = Rc::new(Edge { nodes: (node1.clone(), node2.clone()) });
                edge
            }
        }

        #[derive(Debug)]
        struct Graph {
            nodes: Vec<Rc<Node>>,
            edges: Vec<Rc<Edge>>,
        }

        impl Graph {
            fn new() -> Graph {
                Graph { nodes: Vec::new(), edges: Vec::new() }
            }

        }

        let mut graph = Graph::new();
        let n1 = Node::new("Node 1".to_owned());
        graph.nodes.push(n1.clone());

        let n2 = Node::new("Node 2".to_owned());
        graph.nodes.push(n2.clone());

        let edge = Edge::new(n1.clone(), n2.clone());
        graph.edges.push(edge.clone());

        println!("{:?}", n1);
        println!("{:?}", edge);
        println!("{:?}", graph);

        println!("reference count of n1: {:?}", Rc::strong_count(&n1));
    }

    #[test]
    #[allow(unused)]
    fn rc_deallocate_graph_elements() {

        #[derive(Debug)]
        struct Node {
            label: String,
        }

        impl Node {
            fn new(label: String) -> Rc<Node> {
                Rc::new(Node { label })
            }
        }

        #[derive(Debug)]
        struct Edge {
            nodes: (Rc<Node>, Rc<Node>),
        }

        impl Edge {
            fn new(node1: Rc<Node>, node2: Rc<Node>) -> Rc<Edge> {
                let edge = Rc::new(Edge { nodes: (node1.clone(), node2.clone()) });
                edge
            }
        }

        impl Drop for Edge {
            fn drop(&mut self) {
                // dummy implementation just to illustrate when the instance is dropped
                println!("Dropping edge: {:?}", self)
            }
        }

        #[derive(Debug)]
        struct Graph {
            nodes: Vec<Rc<Node>>,
            edges: Vec<Rc<Edge>>,
        }

        impl Graph {
            fn new() -> Graph {
                Graph { nodes: Vec::new(), edges: Vec::new() }
            }

            fn pop_edge(&mut self) -> Option<Rc<Edge>> {
                self.edges.pop()
            }
        }

        let mut graph = Graph::new();
        let n1 = Node::new("Node 1".to_owned());
        graph.nodes.push(n1.clone());

        let n2 = Node::new("Node 2".to_owned());
        graph.nodes.push(n2.clone());
        {
            let edge = Edge::new(n1.clone(), n2.clone());
            graph.edges.push(edge.clone());
            println!("reference count of edge inside block: {:?}", Rc::strong_count(graph.edges.iter().next().unwrap()));
        }

        println!("reference count of edge outside block: {:?}", Rc::strong_count(graph.edges.iter().next().unwrap()));

        println!("Removing edge from graph...");
        graph.pop_edge();

    }

    #[test]
    // from https://doc.rust-lang.org/book/ch15-05-interior-mutability.html
    fn refcell_intro() {
        trait Messenger {
            fn send(&self, msg: &str);
        }

        struct LimitTracker<'a, T: Messenger> {
            messenger: &'a T,
            value: usize,
            max: usize,
        }

        impl<'a, T> LimitTracker<'a, T>
            where
                T: Messenger,
        {
            pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
                LimitTracker {
                    messenger,
                    value: 0,
                    max,
                }
            }

            pub fn set_value(&mut self, value: usize) {
                self.value = value;

                let percentage_of_max = self.value as f64 / self.max as f64;

                if percentage_of_max >= 1.0 {
                    self.messenger.send("Error: You are over your quota!");
                } else if percentage_of_max >= 0.9 {
                    self.messenger
                        .send("Urgent warning: You've used up over 90% of your quota!");
                } else if percentage_of_max >= 0.75 {
                    self.messenger
                        .send("Warning: You've used up over 75% of your quota!");
                }
            }
        }

        struct MockMessenger {
            sent_messages: RefCell<Vec<String>>,
        }

        impl MockMessenger {
            fn new() -> MockMessenger {
                MockMessenger {
                    sent_messages: RefCell::new(vec![]),
                }
            }
        }

        impl Messenger for MockMessenger {
            fn send(&self, message: &str) {
                self.sent_messages.borrow_mut().push(String::from(message));
            }
        }


        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);

    }

    #[test]
    #[allow(unused)]
    fn refcell_interior_mutability() {

        #[derive(Debug)]
        struct Node {
            label: String,
            edges: RefCell<Vec<Rc<Edge>>>
        }

        impl Node {
            fn new(label: String) -> Rc<Node> {
                Rc::new(Node { label, edges : RefCell::new(Vec::new()) })
            }
        }

        impl Display for Node {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "Node {{ label : {}, edges len: {} }}", self.label, self.edges.borrow().len())
            }
        }

        #[derive(Debug)]
        struct Edge {
            nodes: (Rc<Node>, Rc<Node>),
        }

        impl Edge {
            fn new(node1: Rc<Node>, node2: Rc<Node>) -> Rc<Edge> {
                let edge = Rc::new(Edge { nodes: (node1.clone(), node2.clone()) });
                node1.edges.borrow_mut().push(edge.clone());
                node2.edges.borrow_mut().push(edge.clone());
                edge
            }
        }

        impl Display for Edge {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                write!(f, "Edge {{ nodes: ( {}, {} ) }}", *self.nodes.0, *self.nodes.1)
            }
        }

        #[derive(Debug)]
        struct Graph {
            nodes: Vec<Rc<Node>>,
            edges: Vec<Rc<Edge>>,
        }

        impl Graph {
            fn new() -> Graph {
                Graph { nodes: Vec::new(), edges: Vec::new() }
            }

            fn pop_edge(&mut self) -> Option<Rc<Edge>> {
                self.edges.pop()
            }
        }

        let mut graph = Graph::new();
        let n1 = Node::new("Node 1".to_owned());
        graph.nodes.push(n1.clone());

        let n2 = Node::new("Node 2".to_owned());
        graph.nodes.push(n2.clone());

        let edge = Edge::new(n1.clone(), n2.clone());
        graph.edges.push(edge.clone());

        println!("{}", n1);
        println!("{}", graph.edges.iter().next().unwrap());

        let edge = Edge::new(n1.clone(), n2.clone());
        graph.edges.push(edge.clone());
        println!("{}", n1);

        println!("Removing edge from graph...");
        graph.pop_edge();
    }
}
