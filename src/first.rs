use std::mem;

pub enum List0 {
    Empty,
    Element(i32, Box<List>),
}

struct Node1 {
    elem: i32,
    next: List,
}

pub enum List1 {
    Empty,
    More(Box<Node>),
}

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
    fn new() -> Self {
        Self { head: Link::Empty }
    }

    fn push(&mut self, val: i32) {
        let new_link = Link::More(Box::new(Node {
            elem: val,
            next: mem::replace(&mut self.head, Link::Empty),
        }));
        self.head = new_link;
    }

    fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty){
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut current_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = current_link{
            current_link = mem::replace(&mut boxed_node.next, Link::Empty);

        }
    }
}



#[cfg(test)]
mod test {
    use crate::first::List;
    
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}