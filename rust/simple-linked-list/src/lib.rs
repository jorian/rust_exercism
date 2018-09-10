pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: i32,
}

impl<T> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None,
            size: 0
        }
    }

    pub fn len(&self) -> usize {
        self.size as usize
    }

    pub fn push(&mut self, _element: T) {

        // mem::replace is actually really useful here: the new node we are trying to push
        // has to be the head of the linked list. That's what we see in the last line, where
        // self.head is assigned the new node. But the current head needs to be removed, and it needs
        // to become the next of the new_node we're trying to push.

        // the mem::replace function returns whatever you want replaced; in this case the current
        // head. The current head needs to become the next of the new_node.

        // And before we can assign a new head, we need to remove it first, so None replaces self.head.

        // &mut self is great for methods that want to mutate self.

        let new_node = Box::new(Node {
            data: _element,
            //next: mem::replace(&mut self.head, None)

            // much easier:
            // `take`s the value out of the option, leaving a None in its place.
            next: self.head.take()
        });

        self.head = Some(new_node);
        self.size += 1;
    }

    pub fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            Some(node) => {
                let to_return = *node;

                // we want to move ownership to self.head (since it became the new head)
                self.head = to_return.next;
                self.size -= 1;
                Some(to_return.data)
            }
            None => None
        }
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map( |node| &node.data)
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut result = SimpleLinkedList::new();
        let mut cur_link = &self.head;
        while let Some(i) = cur_link {
            result.push(i.data.clone());
            cur_link = &i.next;
        }
        result
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_item: &[T]) -> Self {
        let mut result = SimpleLinkedList::new();

        for i in _item.iter() {
            result.push(i.to_owned());
        }
        result
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(self) -> Vec<T> {
        let mut result = Vec::new();
        let mut cur_link = self;
        while let Some(i) = cur_link.pop() {
            result.insert(0, i);
        }
        result
    }
}

struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}