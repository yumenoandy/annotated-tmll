#[derive(Debug)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct Node<T> {
    elem: T,
    next: Option<Box<Node<T>>>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None }
    }
    pub fn push(&mut self, elem: T) {
        let node = Box::new(Node {
            elem,
            next: self.head.take(),
        });
        self.head = Some(node);
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

#[cfg(test)]
mod test {
    use super::LinkedList;

    #[test]
    fn basics() {
        let mut list = LinkedList::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);
        println!("{:?}", list);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);
        println!("{:?}", list);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);
        println!("{:?}", list);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));
        println!("{:?}", list);

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
        println!("{:?}", list);
    }
}
