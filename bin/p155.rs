struct MinStack {
    inner: Vec<i32>,
    min_st: Vec<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self {
            inner: vec![],
            min_st: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.inner.push(val);
        match self.min_st.last() {
            Some(last_min) => {
                if val <= *last_min {
                    self.min_st.push(val);
                }
            }
            None => {
                self.min_st.push(val);
            }
        }
    }

    fn pop(&mut self) {
        let val = self.inner.pop().expect("MinStack is empty");
        self.min_st.pop_if(|m| val <= *m);
    }

    fn top(&self) -> i32 {
        *self.inner.last().expect("MinStack is empty")
    }

    fn get_min(&self) -> i32 {
        *self.min_st.last().expect("MinStack is empty")
    }
}

/**
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
fn main() {
    todo!();
}
