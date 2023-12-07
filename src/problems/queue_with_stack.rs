struct MyQueue {
    queue: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */

/**
 * Your MyQueue object will be instantiated and called as such:
 * let obj = MyQueue::new();
 * obj.push(x);
 * let ret_2: i32 = obj.pop();
 * let ret_3: i32 = obj.peek();
 * let ret_4: bool = obj.empty();
 */
impl MyQueue {
    fn new() -> Self {
        MyQueue { queue: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        self.queue.push(x)
    }

    fn pop(&mut self) -> i32 {
        self.queue.remove(0)
    }

    fn peek(&self) -> i32 {
        self.queue.first().unwrap().to_owned()
    }

    fn empty(&self) -> bool {
        self.queue.is_empty()
    }
}
