//struct Node<'a> {
#[derive(Debug)]
struct Node {
    value : i32,
    //next : Option<&'a Node<'a>>
    next : Option<Box<Node>>
}
/*
impl<'a> Node<'a> {
    pub append(&self, value: i32) {
        let mut a = Node {
            value,
            next : None
        };

        while 
    }
}
*/
#[derive(Debug)]
struct LinkedList {
    head: Option<Box<Node>>
}

impl LinkedList{
    fn append_imp(node: &mut Box<Node>, value: i32) {
        match &mut node.next {
            Some(boxed_node) => LinkedList::append_imp(boxed_node, value),
            None => node.next = Some(Box::new( Node{value, next:None} )),
        }
    }

    pub fn append(&mut self, value: i32) {
        match &mut self.head {
            Some(boxed_node) => {
                println!("Got some node");
                LinkedList::append_imp(boxed_node, value);
            }
            None => {
                println!("Got None");
                self.head = Some(Box::new(Node{value, next: None}));
            }
        }
    }

    pub fn new() -> LinkedList {
        LinkedList {head: None}
    }
}
fn main() {
    println!("Hello, world!");

    let mut linked_list = LinkedList::new();

    println!("{:?}", linked_list);
    linked_list.append(1);
    println!("{:?}", linked_list);
    linked_list.append(2);
    println!("{:?}", linked_list);

    /*
    let head: Option<Node> = None;
    let mut a = Node{
        value: 23,
        next : None
    };
    let mut b = Node{
        value: 24,
        next : None
    };
    a.next = Some(&b);
*/
}

