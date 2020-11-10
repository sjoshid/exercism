enum NodeType<'a, V> {
    Middle(Node<'a, V>),
    Leaf,
}

#[derive(Default)]
pub struct Node<'a, V> {
    next_node: Option<&'a NodeType<'a, V>>,
    prev_node: Option<&'a NodeType<'a, V>>,
    value: V,
}

pub(crate) struct DLinkedList<'a, V> {
    head: Option<&'a Node<'a, V>>
}

impl <'a, V> DLinkedList<'a, V> {
    pub(crate) fn new() -> Self {
        DLinkedList {
            head: None
        }
    }

    pub fn add(&mut self, value: V) {
        let new_node = Node {
            prev_node: None,
            next_node: None,
            value,
        };

        self.head = Some(&new_node);
    }
}