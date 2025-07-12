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
        assert!(cap > 0_usize, "Attempt to initialize 0 size buffer");
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

    pub fn peek_from_end(&self, len_from_tail: usize) -> Option<&T> {
        let i = if len_from_tail > self.tail {
            if self.len != self.cap {
                return None;
            }
            self.cap - (len_from_tail - self.tail)
        } else {
            self.tail - len_from_tail
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

    #[test]
    fn test_peek_from_end() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(10_usize);
        buffer.insert(1_i32);
        assert_eq!(buffer.peek_from_end(3_usize), None);

        buffer.insert(2_i32);
        buffer.insert(3_i32);
        buffer.insert(4_i32);

        assert_eq!(buffer.peek_from_end(2_usize), Some(&2_i32));

        buffer.insert(5_i32);
        buffer.insert(6_i32);
        buffer.insert(7_i32);
        buffer.insert(8_i32);
        buffer.insert(9_i32);
        buffer.insert(10_i32);
        buffer.insert(11_i32);
        buffer.insert(12_i32);
        buffer.insert(13_i32);
        assert_eq!(buffer.peek_from_end(2_usize), Some(&11_i32));
        assert_eq!(buffer.peek_from_end(5_usize), Some(&8_i32));
    }
}
