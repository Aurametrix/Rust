pub struct Node {
    value: u64,
    next: Option<Box<Node>>,
    prev: Option<Box<Node>>,
}
// must be mut to be modifiable
let mut head = Node {
    value: 5,
    next: None,
    prev: None,
};
let next = Node {
    value: 6,
    next: None,
                  // next takes ownership of head!!!
    prev: Some(Box::new(head)),
};
// I actually don't understand why the line below compiles. 
// Since `head` was moved into the box, I'm not sure why I can mutate it.
head.next = Some(Box::new(next));

Node {
    // ...
    next: Option<&Box<Node>>
}
