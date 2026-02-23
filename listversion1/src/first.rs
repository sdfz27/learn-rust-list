use std::mem;

//below naive implementation of list will not work because it is recursive and will cause stack overflow
// error[E0072]: recursive type `List` has infinite size
//  --> listversion1\src\first.rs:1:1
//   |
// 1 | pub enum List {
//   | ^^^^^^^^^^^^^
// 2 |     Empty,
// 3 |     Elem(i32, List),
//   |               ---- recursive without indirection
// pub enum List {
//     Empty,
//     Elem(i32, List),
// }
#[derive(Debug)]
pub struct List {
    head: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> List {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    //explain why have to use mem::replace(&mut self.head, Link::Empty) instead of just self.head = Link::Empty
    //because self.head = Link::Empty will move the value of self.head and we will not be able to use it again
    //mem::replace(&mut self.head, Link::Empty) will return the value of self.head and we will be able to use it again
    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
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
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Empty
            // so no unbounded recursion occurs.
        }
    }
}

mod test {
    use super::List;

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
