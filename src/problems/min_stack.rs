struct MinStack {
    stack: Vec<i32>,
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 *  ["MinStack","push","push","push","getMin","pop","top","getMin"]
 [[],[-2],[0],[-3],[],[],[],[]]
 */

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        MinStack { stack: Vec::new() }
    }

    fn push(&mut self, val: i32) {
        self.stack.push(val)
    }

    fn pop(&mut self) {
        self.stack.pop().unwrap();
    }

    fn top(&self) -> i32 {
        self.stack.last().unwrap().to_owned()
    }

    fn get_min(&self) -> i32 {
        self.stack.iter().min().unwrap().to_owned()
    }
}
