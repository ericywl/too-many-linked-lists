struct Node<T> {
    elem: T,
    next: Link<T>,
}

enum Link<T> {
    Empty,
    More(Box<Node<T>>),
}

pub struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            // We use std::mem::replace to steal a value out of a borrow by
            // replacing it with another value
            next: std::mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = std::mem::replace(&mut boxed_node.next, Link::Empty);
            // The boxed_node goes out of scope and gets dropped here,
            // but its Node's `next` field has been set to `Link::Empty`
            // so no unbounded recursion occurs.
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn empty_list() {
        let mut list: List<i32> = List::new();

        // Check empty list behaves correctly
        assert_eq!(list.pop(), None);
    }

    #[test]
    fn normal_removal() {
        let mut list = List::new();

        // Populate list
        for i in 1..=3 {
            list.push(i);
        }

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
    }

    #[test]
    fn exhaustion() {
        let mut list = List::new();

        for i in 1..=3 {
            list.push(i);
        }

        let _ = list.pop();
        let _ = list.pop();

        // Push some more
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
