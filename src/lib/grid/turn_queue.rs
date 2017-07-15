use std::collections::VecDeque;
use std::collections::HashSet;

pub struct TurnQueue {
    queue: VecDeque<usize>,
    element_set: HashSet<usize>,
}

impl TurnQueue {
    pub fn new() -> TurnQueue {
        TurnQueue {
            queue: VecDeque::new(),
            element_set: HashSet::new(),
        }
    }
    
    pub fn push(&mut self, index: usize) {
        if !self.element_set.contains(&index) {
            self.element_set.insert(index);
            self.queue.push_back(index);
        }
    }

    pub fn pop(&mut self) -> Option<usize> {
        match self.queue.pop_front() {
            Some(n) => {
                self.element_set.remove(&n);
                return Some(n);
            },
            None => None
        }
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn contains(&self, index: usize) -> bool {
        self.element_set.contains(&index)
    }
}