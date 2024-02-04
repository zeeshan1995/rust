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

    pub fn insert_after(&mut self, after: i32, value: i32) -> &Option<Box<Node>> {
        /*
        let node = self.find(after); //&Option<Box<Node>>
        //let &mut node: &mut Option<Box<Node>> = self.find(after); //&Option<Box<Node>>
        //match self.find(after) {
        match node {
            //Some(boxed_node) => { //&Box<Node>
            //Some(ref boxed_node) => { //&Box<Node>
            Some(ref mut boxed_node) => { //&mut Box<Node>
                //let cur_next = &boxed_node.next;
                //let new = Some(Box::new(Node{value, next:boxed_node.next}));
                /*
                match &mut boxed_node.next {
                    Some(next) => {
                        boxed_node.next = Some(Box::new(Node{value, next:Some(*next)}));
                        boxed_node.next
                    },
                    None => {
                        boxed_node.next = Some(Box::new(Node{value, next:None}));
                        boxed_node.next
                    }
                }
                */
                if boxed_node.next.is_some() {
                    //boxed_node.next = Some(Box::new(Node{value, next:Some(boxed_node.next.unwrap())}));
                    //boxed_node = Box::new(Node{value, next:Some(boxed_node.next.unwrap())});
                    //let mut n = &boxed_node.next;
                    //let cur: &mut Box<Node> = boxed_node;
                    //let mut cur : &mut Box<Node> = &mut boxed_node;
                    //cur.next = None;//;Some(Box::new(Node{value, next:None}));
                    boxed_node.next = Some(Box::new(Node{value, next:None}));
                }
                //else {
                //    boxed_node.next = Some(Box::new(Node{value, next:None}));
                //}
                &boxed_node.next
            }
            _ => &None
        }
    */
        //Since find returns an immutable reference, can use it here.
        let mut cur = &mut self.head;
        while let Some(node) = cur {
            if node.value == after {
                let new_node = Box::new(Node { value, next: node.next.take(), });
                node.next = Some(new_node);
                cur = &mut node.next;
                break
            }
            cur = &mut node.next;
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

    println!("\n============LinkedList::insert_after=============\n");
    linked_list.insert_after(1, 10);
    println!("{:?}", linked_list);

    linked_list.insert_after(2, 20);
    println!("{:?}", linked_list);

    linked_list.insert_after(3, 30);
    println!("{:?}", linked_list);

    println!("{:?}", linked_list.insert_after(4, 40));
}
