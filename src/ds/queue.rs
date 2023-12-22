struct RecentCounter<i32> {
    ping: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl RecentCounter {
    fn new() -> Self {
        RecentCounter { ping: Vec::new() }
    }

    fn ping(&self, t: i32) -> i32 {
        self.ping.push(t);
        self.ping.len()
    }
}/**
 * Your RecentCounter object will be instantiated and called as such:
 * let obj = RecentCounter::new();
 * let ret_1: i32 = obj.ping(t);
 */
