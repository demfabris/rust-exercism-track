use std::iter::FromIterator;

#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None }
    }

    pub fn is_empty(&self) -> bool {
        if let Some(_) = &self.head {
            false
        } else {
            true
        }
    }

    pub fn len(&self) -> usize {
        let mut head = &self.head;
        let mut count = 0;

        while let Some(node) = head {
            head = &node.next;
            count += 1;
        }

        count
    }

    pub fn push(&mut self, _element: T) {
        self.head = Some(Box::new(Node {
            data: _element,
            next: self.head.take(),
        }))
    }

    pub fn pop(&mut self) -> Option<T> {
        if let Some(node) = self.head.take() {
            self.head = node.next;
            Some(node.data)
        } else {
            None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        if let Some(node) = self.head.as_ref() {
            Some(&node.data)
        } else {
            None
        }
    }

    pub fn rev(&mut self) -> &mut SimpleLinkedList<T> {
        let mut prev = None;
        let mut head = self.head.take();

        while let Some(mut node) = head.take() {
            let next = node.next.take();
            node.next = prev.take();
            prev = Some(node);
            head = next;
        }

        self.head = prev.take();

        self
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        let mut list = SimpleLinkedList::new();
        for element in _iter {
            list.push(element)
        }

        list
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
// Given that implementation, converting to a vector is trivial:
//
// let vec: Vec<_> = simple_linked_list.into_iter().collect();
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut vec = vec![];
        let mut head = self.head;

        while let Some(node) = head {
            vec.push(node.data);
            head = node.next;
        }

        vec.reverse();
        vec
    }
}
