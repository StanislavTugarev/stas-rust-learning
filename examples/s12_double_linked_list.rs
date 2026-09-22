use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct DoublyLinklist {
    head: Pointer,
    tail: Pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: Pointer,
    prev: Pointer,
}

type Pointer = Option<Rc<RefCell<Node>>>;

impl DoublyLinklist {
    fn new() -> Self {
        DoublyLinklist {
            head: None,
            tail: None,
        }
    }

    fn add(&mut self, element: i32) {
        let new_head = Node::new(element);

        match self.head.take() {
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head.clone());
                self.head = Some(new_head);
            }

            None => {
                self.tail = Some(new_head.clone());
                self.head = Some(new_head);
            }
        }
    }

    fn add_back(&mut self, element: i32) {
        let new_tail = Node::new(element);

        match self.tail.take() {
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail.clone());
                self.tail = Some(new_tail);
            }

            None => {
                self.tail = Some(new_tail.clone());
                self.head = Some(new_tail)
            }
        }
    }

    fn remove(&mut self) -> Option<i32> {
        if self.head.is_none() {
            println!("List is empty");
            None
        } else {
            let removed_val = self.head.as_ref().unwrap().borrow().element;
            self.head
                .take()
                .map(|old_head| match old_head.borrow_mut().next.take() {
                    Some(new_head) => {
                        new_head.borrow_mut().prev = None;
                        self.head = Some(new_head);
                        self.head.clone()
                    }
                    None => {
                        self.tail = None;
                        println!("List is empty after removal");
                        None
                    }
                });
            Some(removed_val)
        }
    }

    fn remove_back(&mut self) -> Option<i32> {
        if self.tail.is_none() {
            println!("List is empty");
            None
        } else {
            let removed_val = self.tail.as_ref().unwrap().borrow().element;
            self.tail
                .take()
                .map(|old_tail| match old_tail.borrow_mut().prev.take() {
                    Some(new_tail) => {
                        new_tail.borrow_mut().next = None;
                        self.tail = Some(new_tail);
                        self.tail.clone()
                    }
                    None => {
                        self.head = None;
                        println!("List is empty after removal");
                        None
                    }
                });
            Some(removed_val)
        }
    }

    fn print(&self) {
        let mut traversal = self.head.clone();
        while !traversal.is_none() {
            println!("{}", traversal.as_ref().unwrap().borrow().element);
            traversal = traversal.unwrap().borrow().next.clone();
        }
    }
}

impl Node {
    fn new(element: i32) -> Rc<RefCell<Node>> {
        Rc::new(RefCell::new(Node {
            element,
            next: None,
            prev: None,
        }))
    }
}

fn main() {
    let mut list = DoublyLinklist::new();

    list.add(30);
    list.add(32);
    list.add(34);
    list.add(36);
    list.print();

    list.remove();
    println!("After removal");
    list.print();
}
