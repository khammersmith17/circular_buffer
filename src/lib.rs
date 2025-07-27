use std::fmt;
use std::mem::{replace as mem_replace, take as mem_take};

pub struct CircularBuffer<T> {
    buf: Vec<T>,
    cap: usize,
    tail: usize,
    len: usize,
}

impl<T> std::ops::Index<usize> for CircularBuffer<T>
where
    T: Default + Clone,
{
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(
            index < self.len,
            "Index {index} is out of bounds for buffer size: {}",
            self.len
        );

        &self.buf[(self.head() + index) % self.len]
    }
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

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn insert(&mut self, item: T) {
        self.len = std::cmp::min(self.cap, self.len + 1);
        self.tail = (self.tail + 1) % self.cap;
        let _ = mem_replace(&mut self.buf[self.tail], item);
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
        if len_from_tail > self.len {
            return None;
        }
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

#[derive(Debug)]
pub enum QueueError {
    QueueEmpty,
    QueueFull,
}

impl fmt::Display for QueueError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_msg = match self {
            Self::QueueEmpty => "Queue is empty",
            Self::QueueFull => "Queue is full",
        };

        write!(f, "{}", err_msg)
    }
}

impl std::error::Error for QueueError {}

pub struct StaticSizeQueue<T> {
    buff: Vec<T>,
    front: usize,
    size: usize,
    back: usize,
    cap: usize,
}

/// A preallocated queue where space is allocated up front and is circular in nature.
/// Allocated a certain capacity upfront, then the same allocated space is used to push and pop
/// items in the queue.
/// No dynamic allocation of space after initial construction.
/// Type must implement Default + Clone in order to initialize the entire space up front.
/// Entire block is initialized with Default, but not accessible until that space is overwritten
/// with values pushed onto the queue.
impl<T> StaticSizeQueue<T>
where
    T: Default + Clone,
{
    /// Takes in a capacity and a type. Allocates and initializes entire allocated block.
    pub fn new(cap: usize) -> StaticSizeQueue<T> {
        let mut buff: Vec<T> = Vec::with_capacity(cap);
        buff.resize(cap, T::default());
        let front = 0_usize;
        let back = 0_usize;
        let size = 0_usize;
        StaticSizeQueue {
            buff,
            front, // the next item to be poped off the queue
            size,  // the number of items in the queue
            back,  // the space where the next item will be pushed
            cap,   // tjhe capacity of the queue
        }
    }

    /// Pop an item off the front of the queue. Will result a QueueError if the queue is empty.
    /// On success an owned instance on T will be returned.
    pub fn pop(&mut self) -> Result<T, QueueError> {
        if self.is_empty() {
            return Err(QueueError::QueueEmpty);
        }

        let item = mem_take(&mut self.buff[self.front]);
        self.front = (self.front + 1) % self.cap;
        self.size -= 1;
        Ok(item)
    }

    /// Takes an item of type T. If the queue is full, a QueueError will be returned. On success,
    /// an Ok unit type will be returned.
    pub fn push(&mut self, item: T) -> Result<(), QueueError> {
        if self.size == self.cap {
            // queue at capacity
            return Err(QueueError::QueueFull);
        }

        let _ = mem_replace(&mut self.buff[self.back], item);
        self.back = (self.back + 1) % self.cap;
        self.size += 1;
        Ok(())
    }

    /// Returns the number of items in the queue.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Get a shared reference to the item at the front on the queue.
    pub fn peek_front(&self) -> Option<&T> {
        if self.size == 0 {
            return None;
        }

        Some(&self.buff[self.front])
    }

    /// Get a mutable reference to the item at the front of the queue.
    pub fn peek_front_mut(&mut self) -> Option<&mut T> {
        if self.size == 0 {
            return None;
        }

        Some(&mut self.buff[self.front])
    }

    /// Utility to see if there are items in the queue
    pub fn is_empty(&self) -> bool {
        self.size == 0
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
        assert_eq!(buffer.peek_from_end(1_usize), Some(&12_i32));
    }

    #[test]
    fn test_peek_from_end_loop() {
        let mut buffer: CircularBuffer<i32> = CircularBuffer::new(10_usize);
        buffer.insert(1_i32);

        buffer.insert(2_i32);
        buffer.insert(3_i32);
        buffer.insert(4_i32);

        buffer.insert(5_i32);
        buffer.insert(6_i32);
        buffer.insert(7_i32);
        buffer.insert(8_i32);
        buffer.insert(9_i32);
        buffer.insert(10_i32);
        buffer.insert(11_i32);
        buffer.insert(12_i32);
        buffer.insert(13_i32);
        let mut end = 0_usize;
        while let Some(ticker) = buffer.peek_from_end(end) {
            println!("{ticker}");
            end += 1;
        }
    }

    #[test]
    fn basic_queue_test() {
        let mut queue: StaticSizeQueue<i32> = StaticSizeQueue::new(10_usize);
        let _ = queue.push(1_i32);
        assert_eq!(queue.size(), 1_usize);

        let _ = queue.push(2_i32);
        let _ = queue.push(3_i32);
        let _ = queue.push(4_i32);

        assert_eq!(queue.size, 4_usize);

        let _ = queue.push(5_i32);
        let _ = queue.push(6_i32);
        let _ = queue.push(7_i32);
        let _ = queue.push(8_i32);
        let _ = queue.push(9_i32);
        let _ = queue.push(10_i32);
        let res = queue.push(11_i32);
        assert!(res.is_err());

        let item = queue.pop();
        assert_eq!(item.unwrap(), 1_i32);

        let item = queue.pop();
        assert_eq!(item.unwrap(), 2_i32);

        let item = queue.pop();
        assert_eq!(item.unwrap(), 3_i32);

        let item = queue.pop();
        assert_eq!(item.unwrap(), 4_i32);

        let item = queue.pop();
        assert_eq!(item.unwrap(), 5_i32);

        let item = queue.pop();
        assert_eq!(item.unwrap(), 6_i32);

        let item = queue.pop();
        assert_eq!(item.unwrap(), 7_i32);

        let item = queue.pop();
        assert_eq!(item.unwrap(), 8_i32);

        let item = queue.pop();
        assert_eq!(item.unwrap(), 9_i32);

        let item = queue.pop();
        assert_eq!(item.unwrap(), 10_i32);

        assert!(queue.is_empty());

        queue.push(13_i32).unwrap();
        assert_eq!(queue.size(), 1_usize);
        let item = queue.pop();
        assert_eq!(item.unwrap(), 13_i32);
    }
}
