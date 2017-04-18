struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

struct List<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    fn new() -> List<T> {
        List { head: None }
    }

    fn push(&mut self, elem: T) {
        self.head = Some(box Node { 
            data: elem, 
            next: self.head.take(), 
        });
    }

    fn pop(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(mut head) => {
                self.head = head.next.take();
                Some(head.data)
            }
        }
    }
}
