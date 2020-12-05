pub struct Queue<T> {
    items: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    pub fn enqueue(&mut self, item: T) {
        self.items.insert(0, item);
    }

    pub fn dequeue(&mut self) -> Option<T> {
        self.items.pop()
    }

    pub fn count(self) -> usize {
        self.items.len()
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_retains_order() {
        let mut q = Queue::new();
        q.enqueue(1);
        q.enqueue(2);
        q.enqueue(3);

        assert_eq!(q.dequeue().unwrap(), 1);
        assert_eq!(q.dequeue().unwrap(), 2);
        assert_eq!(q.dequeue().unwrap(), 3);
    }
}
