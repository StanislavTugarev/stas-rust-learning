#[derive(Debug)]
struct Linklist {
    head: Pointer,
}

#[derive(Debug)]
struct Node {
    element: i32,
    next: Pointer,
}

type Pointer = Option<Box<Node>>;

impl Linklist {
    fn new() -> Linklist {
        Linklist { head: None }
    }

    fn add(&mut self, element: i32) {
        // match self.head {
        //     None => {
        //         let new_node = Some(Box::new(Node {
        //             element: element,
        //             next: None,
        //         }));
        //         self.head = new_node;
        //     }
        //     Some(previous_head) => {
        //         let new_node = Some(Box::new(Node {
        //             element: element,
        //             next: Some(previous_head),
        //         }));
        //     }
        // }

        let previous_head = self.head.take();
        let new_head = Some(Box::new(Node {
            element: element,
            next: previous_head,
        }));
        self.head = new_head;
    }

    fn remove(&mut self) -> Option<i32> {
        match self.head.take() {
            Some(previous_head) => {
                self.head = previous_head.next;
                Some(previous_head.element)
            }
            None => None,
        }
    }

    fn print(&self) {
        let mut list_traversal = &self.head;
        while !list_traversal.is_none() {
            println!("{:?}", list_traversal.as_ref().unwrap().element);
            list_traversal = &list_traversal.as_ref().unwrap().next;
        }
    }
}

fn main() {
    let list = Node {
        element: 1,
        next: None,
    };

    let list = Node {
        element: 1,
        next: Some(Box::new(Node {
            element: 2,
            next: None,
        })),
    };

    // let list = Linklist {
    //     head: Some(Node {
    //         element: 1,
    //         next: None,
    //     }),
    // };

    // let list = Linklist {
    //     head: Some(Node {
    //         element: 1,
    //         next: Some(Box::new(Node {
    //             element: 2,
    //             next: Some(Box::new(Node {
    //                 element: 3,
    //                 next: None,
    //             })),
    //         })),
    //     }),
    // };

    let list = Linklist { head: None };

    let list = Linklist {
        head: Some(Box::new(Node {
            element: 100,
            next: Some(Box::new(Node {
                element: 200,
                next: None,
            })),
        })),
    };

    println!("{:?}", &list.head.unwrap().next.unwrap().element);

    let mut list = Linklist::new();
    list.add(5);
    list.add(7);
    list.add(10);
    list.add(15);
    list.add(20);

    println!("List: {:?}", list);
    println!("{}", list.remove().unwrap());

    list.print();
}
