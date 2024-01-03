#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Node {
    value: Option<i32>,
    next: Option<Box<Node>>,
}

impl Node {
    fn new() -> Node {
        Node {
            value: None,
            next: None,
        }
    }

    fn add_node(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value: Some(value),
            next: None,
        });

        let mut current = self;
        while let Some(ref mut next) = current.next {
            current = next;
        }

        current.next = Some(new_node);
    }

    fn print_nodes(&self) {
        let mut node = self;

        while let Some(ref next) = node.next {

            node = next;
            if let Some(value) = node.value {
                println!("{:?}", value);
            }

        }

    }
}

fn main() {
    let mut node = Node::new();

    node.add_node(1);
    node.add_node(2);
    node.add_node(3);

    node.print_nodes();
}
