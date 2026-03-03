struct MinStack {
    st: Vec<i64>,
    minv: i64,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    fn new() -> Self {
        Self {
            st: vec![],
            minv: 0,
        }
    }

    fn push(&mut self, val: i32) {
        if self.st.is_empty() {
            self.st.push(0);
            self.minv = val as i64;
        } else {
            let x = val as i64;
            let diff = x - self.minv;
            self.st.push(diff);
            if diff < 0 {
                self.minv = x;
            }
        }
    }

    fn pop(&mut self) {
        let diff = self.st.pop().unwrap();
        if diff < 0 {
            self.minv = self.minv - diff;
        }
    }

    fn top(&self) -> i32 {
        let diff = *self.st.last().unwrap();
        let x = if diff >= 0 {
            self.minv + diff
        } else {
            self.minv
        };
        x as i32
    }

    fn get_min(&self) -> i32 {
        self.minv as i32
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
fn main() {}
