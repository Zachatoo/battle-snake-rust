use std::collections::VecDeque;

pub struct FifoQueue<T> {
    queue: VecDeque<T>,
}

impl<T> FifoQueue<T> {
    pub fn new() -> FifoQueue<T> {
        FifoQueue::<T> {
            queue: VecDeque::<T>::new(),
        }
    }

    pub fn enqueue(&mut self, item: T) {
        self.queue.push_back(item)
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    pub fn clear(&mut self) {
        self.queue.clear();
    }
}
