use std::borrow::Borrow;
use std::iter::FromIterator;
use std::mem;
use std::ops::Deref;


enum NodeType<T> {
    Leaf,
    Internal(Box<Node<T>>),
}

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
        SimpleLinkedList {
            head: NodeType::Leaf,
            size: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.size
    }

    pub fn push(&mut self, element: T) {
        let new_end_node = Box::new(Node {
            data: element,
            next: mem::replace(&mut self.head, NodeType::Leaf),
        });

        self.head = NodeType::Internal(new_end_node);

        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match mem::replace(&mut self.head, NodeType::Leaf) {
            NodeType::Internal(bb) => {
                self.head = bb.next;
                self.size -= 1;
                Some(bb.data)
            }
            NodeType::Leaf => {
                println!("here");
                None
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            NodeType::Internal(bb) => {
                Some(&bb.data)
            },
            NodeType::Leaf => {
                None
            }
        }
    }

    pub fn rev(self) -> SimpleLinkedList<T> {
        let mut revved_list = SimpleLinkedList::new();

        let mut current_head = self.head;
        while let NodeType::Internal(bb) = current_head {
            let value = bb.data;
            revved_list.push(value);
            current_head = bb.next;
        }

        revved_list
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
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
    fn into(self) -> Vec<T> {
        let mut result = Vec::with_capacity(self.size);
        let mut current_head = self.head;

        // It is better to add element from back of vec so I dont have to reverse it.
        // But to do that all I can think of is maintaining an index that keeps decrementing.
        // That doesnt feel very....rusty.
        // Is there a rusty way to do this?
        while let NodeType::Internal(bb) = current_head {
            let value = bb.data;
            result.push(value);
            current_head = bb.next;
        }

        result.reverse();
        result
    }
}
