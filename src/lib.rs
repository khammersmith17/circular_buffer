pub struct CircularBuffer<T> {
    buf: Vec<T>,
    cap: usize,
    tail: usize,
    len: usize,
}

impl<T> CircularBuffer<T>
where
    T: Default + Clone,
{
    pub fn new(cap: usize) -> CircularBuffer<T> {
        assert!(cap <= 0_usize, "Attempt to initialize 0 size buffer");
        let mut buf: Vec<T> = Vec::with_capacity(cap);
        buf.resize(cap, T::default());
        CircularBuffer {
            buf,
            tail: cap - 1_usize,
            len: 0_usize,
            cap,
        }
    }

    pub fn insert(&mut self, item: T) {
        self.len = std::cmp::min(self.cap, self.len + 1);
        self.tail = (self.tail + 1) % self.cap;
        self.buf[self.tail] = item;
    }

    pub fn peek_tail(&self) -> Option<&T> {
        if self.len > 0 {
            Some(&self.buf[self.tail])
        } else {
            None
        }
    }

    pub fn head(&self) -> usize {
        (self.tail + self.len + 1) % self.len
    }

    pub fn peek_from_end(&self, length_from_end: usize) -> Option<&T> {
        if self.len == 1 {
            return None;
        }

        let i = if self.tail == 0 {
            self.cap - 1
        } else {
            self.tail - 1
        };

        Some(&self.buf[i])
    }

    pub fn peek_head(&self) -> Option<&T> {
        if self.len > 0 {
            Some(&self.buf[self.head()])
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_head_comp_overflow() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(10_usize);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        let head = buffer.head();

        // 12 items
        // tail is index 1
        // head should be tail + 1 when it is wrapped
        assert_eq!(head, 2_usize);
    }

    #[test]
    fn test_head_comp_not_full() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(10_usize);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        let head = buffer.head();

        assert_eq!(head, 0_usize);
    }

    #[test]
    fn test_head_comp_edge() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(10_usize);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        let head = buffer.head();

        // 11 items
        // tail is index 0
        // head should be tail + 1 when it is wrapped
        assert_eq!(head, 1_usize);
    }

    #[test]
    fn test_head_comp_edge2() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(10_usize);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        buffer.insert(1_i32);
        let head = buffer.head();

        // 12 items
        // tail is index 1
        // head should be tail + 1 when it is wrapped
        assert_eq!(head, 0_usize);
    }
}
