pub mod linked_list {
    use std::cell::RefCell;
    use std::rc::Rc;

    #[derive(Debug, Clone)]    
    pub struct Node {
        pub value: String,
        pub next: Link,
        pub prev: Link
    }

    type Link = Option<Rc<RefCell<Node>>>;

    impl Node {
        // a nice and short way of creating a new node
        pub fn new(value: String) -> Rc<RefCell<Node>> {
            Rc::new(RefCell::new(Node {
                value,
                next: None,
                prev: None,
            }))
        }
    }

    pub struct LinkedListIterator {
        current: Link,
    }

    impl LinkedListIterator {
        fn new(start_at: Link) -> Self {
            LinkedListIterator {
                current: start_at,
            }
        }
    }

    impl Iterator for LinkedListIterator {
        type Item = String;
        fn next(&mut self) -> Option<String> {
            let current = &self.current;
            let mut result = None;
            self.current = match current {
                Some(ref current) => {
                    let current = current.borrow();
                    result = Some(current.value.clone());
                    current.next.clone()
                },
                None => None
            };
           result
        }
    }

    impl DoubleEndedIterator for LinkedListIterator {
        fn next_back(&mut self) -> Option<String> {
            let current = &self.current;
            let mut result = None;
            self.current = match current {
                Some(ref current) => {
                    let current = current.borrow();
                    result = Some(current.value.clone());
                    current.prev.clone()
                },
                None => None
            };
           result
        }
    }

    #[derive(Debug, Clone)]    
    pub struct BetterTransactionLog {
        pub head: Link,
        pub tail: Link,
        pub length: u64
    }

    impl BetterTransactionLog {
        pub fn new_empty() -> Self {
            BetterTransactionLog {
                head: None,
                tail: None,
                length: 0
            }
        }

        pub fn append(&mut self, value: String){
            let new = Node::new(value);
            match self.tail.take(){
                Some(old) => old.borrow_mut().next = Some(new.clone()),
                None => self.head = Some(new.clone())
            };
            self.length += 1;
            self.tail= Some(new);
        }

        pub fn pop(&mut self) -> Option<String> {
            self.head.take().map(|head|{
                if let Some(next) = head.borrow_mut().next.take(){
                    self.head = Some(next);
                } else {
                    self.tail.take();
                }
                self.length -= 1;
                Rc::try_unwrap(head)
                     .ok()
                     .expect( "An error has happened!")
                     .into_inner()
                     .value
            })
        }
        
        pub fn get_size(&self) -> u64 {
                self.length.clone()
       }
        
    }
}