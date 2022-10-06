use std::collections::VecDeque;

/// Monotonic Queue is a data structure, where the elements from front to end
/// are either strictly increasing or decreasing. For example, a strictly increasing
/// monotonic queue will be able to contain `[1, 3, 4, 6, 7]`, but not `[1, 1, 3].
///
/// There are two types of monotonoic queue, increasing or decreasing.
/// - Monotonic increasing queue: to push an element e, starts from the rear element
///   we pop out element (s >= e) (violation);
/// - Monotonic decreasing queue: we pop out element s <= e (violation)
/// - Sometimes we can relax the strict monotonic condition, and can allow the stack
///   or queue have duplicate value.
///
pub struct MonotonicQueue<T> {
    dq: VecDeque<T>,
}

impl<T> MonotonicQueue<T> {
    /// Create an empty monotonic queue.
    ///
    /// # Example
    /// ```
    /// use monotonicqueue::MonotonicQueue;
    ///
    /// let mq: MonotonicQueue<i32> = MonotonicQueue::new();
    /// ```
    pub fn new() -> MonotonicQueue<T> {
        MonotonicQueue {
            dq: VecDeque::new(),
        }
    }

    /// Provides a peek to the front element, or None.
    ///
    /// # Example
    /// ```
    /// use monotonicqueue::MonotonicQueue;
    ///
    /// let mut mq = MonotonicQueue::new();
    ///
    /// let is_less = |n1: &i32, n2:&i32| n1.lt(n2);
    /// mq.push_by(1, is_less);
    /// mq.push_by(2, is_less);
    ///
    /// assert_eq!(mq.peek(), Some(&2));
    /// ```
    pub fn peek(&self) -> Option<&T> {
        self.dq.front()
    }

    pub fn pop(&mut self) -> Option<T> {
        self.dq.pop_front()
    }

    pub fn push_by<F>(&mut self, item: T, is_less: F)
    where
        F: Fn(&T, &T) -> bool,
    {
        while let Some(existing_item) = self.dq.back() {
            if is_less(existing_item, &item) {
                self.dq.pop_back();
            } else {
                break;
            }
        }
        self.dq.push_back(item);
    }
}

#[cfg(test)]
mod tests {
    use crate::MonotonicQueue;

    #[test]
    fn monotonic_incresing_queue() {
        let mut mq = MonotonicQueue::new();

        let is_less = |n1: &i32, n2: &i32| n1.gt(n2);
        mq.push_by(1, is_less);
        mq.push_by(2, is_less);

        assert_eq!(mq.peek(), Some(&1));
    }

    #[test]
    fn monotonic_decreasing_queue() {
        let mut mq = MonotonicQueue::new();

        let is_less = |n1: &i32, n2: &i32| n1.lt(n2);
        mq.push_by(1, is_less);
        mq.push_by(2, is_less);

        assert_eq!(mq.peek(), Some(&2));
    }
}
