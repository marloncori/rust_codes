
pub mod algorithm {
    extern crate rand;

    use rand::random;
    use std::rc::Rc;
    use std::cell::RefCell;

    type Link = Option<Rc<RefCell<Node>>>;

    #[derive(Clone)]
    pub struct Node {
        pub next: Vec<Link>,
        pub offset: u64,
        pub command: String,
    }

    impl Node {
        pub fn new(next: Vec<Link>, offset: u64, command: String) -> Self {
            Node {
                next, offset, command,
            }
        }
    }

    #[derive(Clone)]
    pub struct SkipList {
        pub head: Link,
        pub tails: Vec<Link>,
        pub max_level: usize,
        pub length: u64,
    }

    impl SkipList {
        pub fn new(head: Link, tails: Vec<Link>,
             max_level: usize, length: u64) -> Self {
                SkipList {
                    head, tails, 
                    max_level, length,
                }
        }

        pub fn append(&mut self, offset: u64, value: String) {
            let level = if self.head.is_none() {
                // use the maximum level for the first node
                self.max_level + 1
            } else {
                // determine the level by coin flips
                self.get_level() + 1
            };

            let new = Rc::new(
                        RefCell::new(
                            Node::new(vec![None; level], offset, value
                            )
                        )
                    );

            //update the tails for each level
            for i in 0..level {
                if let Some(old) = self.tails[i].take() {
                    let next = &mut old.borrow_mut().next;
                    next[i] = Some(new.clone());
                }
                self.tails[i] = Some(new.clone());
            }

            // this is the first node in the list
            if self.head.is_none() {
                self.head = Some(new.clone());
            }

            self.length += 1;
        }

        pub fn get_level(&mut self) -> usize {
            let mut n = 0;
            // bool = p(true) = 0.5
            while rand::random::<bool>() && n < self.max_level {
                n += 1;
            }
            n
        }

        pub fn find(&self, offset: u64) -> Option<String> {
            match self.head {
                Some(ref head) => {
                    let mut start_level= self.max_level;
                    let node = head.clone();
                    let mut result = None;
                    loop {
                        if node.borrow().next[start_level].is_some() {
                            break;
                        }
                        start_level -= 1;
                    }
                    let mut n = node;
                    for level in (0..=start_level).rev() {
                        loop {
                            let next = n.clone();
                            match next.borrow().next[level] {
                                Some(ref next)
                                    if next.borrow().offset <= offset =>
                                        n = next.clone(),
                                 _ =>  break
                            };
                        }
                        if n.borrow().offset == offset {
                            let tmp = n.borrow();
                            result = Some(tmp.command.clone());
                            break;
                        }
                    }
                    result
                }
                None => None,
            }
        }

    }


}