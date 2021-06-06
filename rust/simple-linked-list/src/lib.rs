use std::borrow::Borrow;
use std::iter::FromIterator;
use std::mem;
use std::ops::Deref;
type NodeType<T> = Option<Box<Node<T>>>;

struct Node<T> {
    data: T,
    next: NodeType<T>,
}

pub struct SimpleLinkedList<T> {
    head: NodeType<T>,
    size: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList { head: None, size: 0 }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, element: T) {
        let new_end_node = Box::new(Node {
            data: element,
            next: self.head.take(),
        });

        self.head = Some(new_end_node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            self.size -= 1;
            node.data
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.data
        })
    }

    pub fn rev(mut self) -> SimpleLinkedList<T> {
        let mut previous = None;
        let mut size = 0 as usize;

        while self.head.is_some() {
            let mut h = None;
            let n = self.head.as_mut().unwrap();
            h = n.next.take();
            n.next = previous;
            previous = self.head;
            self.head = h;
            size += 1;
        }

        SimpleLinkedList {
            head: previous,
            size,
        }
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut list = SimpleLinkedList::new();

        for item in iter {
            list.push(item);
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
    fn into(mut self) -> Vec<T> {
        let mut result = Vec::new();

        // It is better to add element from back of vec so I dont have to reverse it.
        // But to do that all I can think of is maintaining an index that keeps decrementing.
        // That doesnt feel very....rusty.
        // Is there a rusty way to do this?
        while let Some(n) = self.pop() {
            result.push(n);
        }

        result.reverse();
        result
    }
}

pub struct IteratorForList<T> {
    current_node: NodeType<T>,
}

impl <T> Iterator for IteratorForList<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.current_node.take().map(|node| {
            self.current_node = node.next;
            node.data
        })
    }
}

impl <T> IntoIterator for SimpleLinkedList<T> {
    type Item = T;
    type IntoIter = IteratorForList<T>;

    fn into_iter(mut self) -> Self::IntoIter {
        IteratorForList {
            current_node: self.head
        }
    }
}
