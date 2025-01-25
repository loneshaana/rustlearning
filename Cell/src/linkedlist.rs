use std::{
    mem::{self, swap, take},
    ptr::NonNull,
};

///! A doubly linked list with owned values
///
/// The `LinkedList` allows pushing and popping elements at either end in constant time
///
/// Note: it is almost always better to use [`Vec`] or [`VecDeque`] because
/// array-based containers are generally faster
/// more memory efficient and make better use of CPU cache.
///
///
struct Node<T> {
    element: T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

pub struct LinkedList<T> {
    head: Option<NonNull<Node<T>>>,
    tail: Option<NonNull<Node<T>>>,
    len: usize,
}

impl<T> Node<T> {
    fn new(element: T) -> Self {
        Self {
            next: None,
            prev: None,
            element,
        }
    }

    fn into_element(self: Box<Self>) -> T {
        self.element
    }
}

impl<T> LinkedList<T> {
    unsafe fn push_front_node(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            // point next of the node to head
            (*node.as_ptr()).next = self.head;

            // set previous of node to None
            (*node.as_ptr()).prev = None;

            let node = Some(node);

            match self.head {
                None => self.tail = node, // If head was None, then node is our tail as well.
                Some(head) => (*head.as_ptr()).prev = node,
            }
            self.head = node;
            self.len += 1;
        }
    }

    fn pop_front_node(&mut self) -> Option<Box<Node<T>>> {
        // This method takes care not to create mutable references to whole nodes,
        // to maintain validity of aliasing pointers into `element`

        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());

            // update ptr: make next of the node as head.
            self.head = node.next;

            match self.head {
                None => self.tail = None,
                Some(head) => (*head.as_ptr()).prev = None,
            }
            self.len -= 1;
            node
        })
    }

    fn push_back_node(&mut self, node: NonNull<Node<T>>) {
        unsafe {
            // add the node as the prev of the linkedlist.
            (*node.as_ptr()).next = None;
            (*node.as_ptr()).prev = self.tail;
            let node = Some(node);
            match self.tail {
                None => self.head = node,
                Some(tail) => (*tail.as_ptr()).next = node,
            }
            self.tail = node;
            self.len += 1;
        }
    }

    fn pop_back_node(&mut self) -> Option<Box<Node<T>>> {
        self.tail.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.tail = node.prev;

            match self.tail {
                None => self.head = None,
                Some(tail) => (*tail.as_ptr()).next = None,
            }
            self.len -= 1;
            node
        })
    }

    fn unlink_node(&mut self, mut node: NonNull<Node<T>>) {
        let node = unsafe { node.as_mut() };

        // next of the previous of the node should point to the next of node
        // the previous of the next of the node should point to the previous of the node

        match node.prev {
            None => self.head = node.next,
            Some(prev) => unsafe { (*prev.as_ptr()).next = node.next },
        }

        match node.next {
            None => self.tail = node.prev,
            Some(next) => unsafe { (*next.as_ptr()).prev = node.prev },
        }
        self.len -= 1;
    }
}

impl<T> LinkedList<T> {
    #[must_use]
    pub fn new() -> Self {
        Self {
            head: None,
            tail: None,
            len: 0,
        }
    }

    // moves all elements from `other` to the end of the list
    // This reuses all the nodes from `other` and moves them into `self`.
    // After this operation, `other` becomes empty

    // This operation should compute in O(1) time and O(1) memory
    pub fn append(&mut self, other: &mut Self) {
        match self.tail {
            None => swap(self, other),
            Some(mut tail) => {
                if let Some(mut other_head) = other.head.take() {
                    unsafe {
                        tail.as_mut().next = Some(other_head);
                        other_head.as_mut().prev = Some(tail);
                    }
                    self.tail = other.tail;
                    self.len += mem::replace(&mut other.len, 0);
                }
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn clear(&mut self) {
        drop(LinkedList {
            head: self.head.take(),
            tail: self.tail.take(),
            len: take(&mut self.len),
        })
    }

    pub fn contains(&self, x: T) -> bool {
        false // @TODO implement iter
    }

    pub fn front(&self) -> Option<&T> {
        unsafe {
            let refhead = self.head.as_ref();
            refhead.map(|node| {
                let refnode = node.as_ref();
                &refnode.element
            })
        }
    }

    pub fn front_mut(&mut self) -> Option<&mut T> {
        unsafe {
            let refhead = self.head.as_mut();
            refhead.map(|node| {
                let refnode = node.as_mut();
                &mut refnode.element
            })
        }
    }

    pub fn back(&self) -> Option<&T> {
        unsafe {
            let refback = self.tail.as_ref();
            refback.map(|node| {
                let refnode = node.as_ref();
                &refnode.element
            })
        }
    }

    pub fn back_mut(&mut self) -> Option<&mut T> {
        unsafe {
            let refback = self.tail.as_mut();
            refback.map(|node| {
                let refnode = node.as_mut();
                &mut refnode.element
            })
        }
    }

    pub fn push_front(&mut self, ele: T) {
        let node = Box::new(Node::new(ele));
        let node_ptr = NonNull::from(Box::leak(node));
        unsafe {
            self.push_front_node(node_ptr);
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        self.pop_front_node().map(|node| node.element)
    }

    pub fn push_back(&mut self, ele: T) {
        let node = Box::new(Node::new(ele));
        let node_ptr = NonNull::from(Box::leak(node));
        self.push_back_node(node_ptr);
    }

    pub fn pop_back(&mut self) -> Option<T> {
        self.pop_back_node().map(|node| node.element)
    }
}

// struct LinkedListIntoIter<T> {
//     list: LinkedList<T>,
// }

// impl<T> Iterator for LinkedListIntoIter<T> {
//     type Item = T;

//     fn next(&mut self) -> Option<T> {
//         self.list.pop_front()
//     }

//     fn size_hint(&self) -> (usize, Option<usize>) {
//         (self.list.len, Some(self.list.len))
//     }
// }

// impl<T> IntoIterator for LinkedList<T> {
//     type Item = T;
//     type IntoIter = LinkedListIntoIter<T>;

//     fn into_iter(self) -> LinkedListIntoIter<T> {
//         LinkedListIntoIter { list: self }
//     }
// }

// impl<T> Iterator for LinkedList<T> {
//     type Item = T;
//     fn next(&mut self) -> Option<Self::Item> {
//         None
//     }
// }
