pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

impl Link {
    pub fn new() -> Self {
        Link { head: Link::Empty }
    }
}

struct Node {
    elem: i32,
    next: Link,
}
