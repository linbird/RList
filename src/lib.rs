pub mod stack_list {
    
    pub struct List {
        head: Link,
    }

    enum Link {
        Empty,
        More(Box<Node>),
    }

    struct Node {
        elem: i32,
        next: Link,
    }
    
    impl List {
        pub fn new() -> List {
            List {
                head: Link::Empty
            }
        }

        pub fn push(&mut self, elem: i32) {
            let new_node = Node {
                elem: elem,
                next: std::mem::replace(&mut self.head, Link::Empty),
            };
            self.head = Link::More(Box::new(new_node));
        }

        pub fn pop(&mut self) -> Option<i32> {
            match std::mem::replace(&mut self.head, Link::Empty) {
                Link::Empty => None,
                Link::More(node) => {
                    self.head = node.next;
                    Some(node.elem)
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::stack_list::List;

    #[test]
    fn basics() {
        let mut list = List::new();
        assert_eq!(list.pop(), None);
        list.push(1);
        list.push(2);
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
