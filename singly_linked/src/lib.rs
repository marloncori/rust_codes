
pub mod linked_list {
    use std::cell::RefCell;
    use std::rc::Rc;

    type Link = Option<Rc<RefCell<Node>>>;

    #[derive(Clone)]    
    pub struct Node {
        pub value: String,
        pub next: Link
    }

    impl Node {
        // a nice and short way of creating a new node
        pub fn new(value: String) -> Rc<RefCell<Node>> {
            Rc::new(RefCell::new(Node {
                value,
                next: None,
            }))
        }
    }

    pub struct TransactionLog {
        pub head: Link,
        pub tail: Link,
        pub length: u64
    }

    impl TransactionLog {
        pub fn new_empty() -> Self {
            TransactionLog {
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