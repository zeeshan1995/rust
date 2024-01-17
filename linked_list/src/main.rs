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

    pub fn find(&mut self, value: i32) -> &Option<Box<Node>> {
        let mut cur = &self.head;
        while cur.is_some() {
            cur = match &mut cur {
                Some(boxed_node) => {
                    match boxed_node.value {
                        x if x == value => return cur,
                        _ => &boxed_node.next
                    }
                },
                None => return cur
            };
        }
        cur
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
    linked_list.append(3);
    println!("{:?}", linked_list);

    println!("\n============LinkedList::find=============\n");
    println!("{:?}", linked_list.find(1));
    println!("{:?}", linked_list.find(2));
    println!("{:?}", linked_list.find(3));
    println!("{:?}", linked_list.find(4));
}

