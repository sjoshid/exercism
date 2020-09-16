use std::iter::FromIterator;
use std::borrow::Borrow;

enum InternalType<T> {
    Internal(Node<T>),
}

struct Node<T> {
    data: T,
    next: Option<Box<InternalType<T>>>,
}

pub struct SimpleLinkedList<T> {
    head: Option<Node<T>>,
    size: usize,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            size: 0
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, element: T) {
        let new_end_node = Node {
            data: element,
            next: None,
        };
        if let Some(mut current) = self.head.as_mut() {
            while current.next.is_some() {
                let ljkadf = &mut **current.next.as_mut().unwrap();
                if let InternalType::Internal(n) = ljkadf {
                    current = n;
                }
            }
            // when I get here that means current.next is None
            current.next = Some(Box::new(InternalType::Internal(new_end_node)));
        } else {
            self.head = Some(new_end_node);
        }

        self.size.checked_add(1);
        //unimplemented!()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.size.checked_rem(1);
        unimplemented!()
    }

    pub fn peek(&self) -> Option<&T> {
        unimplemented!()
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        unimplemented!()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        unimplemented!()
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
        unimplemented!()
    }
}
