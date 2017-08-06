use std::collections::VecDeque;
use std::collections::HashSet;

type QueueElement = (u32, u32, u32);

pub struct TurnQueue {
    queue: VecDeque<QueueElement>,
    element_set: HashSet<QueueElement>,
}

impl TurnQueue {
    pub fn new() -> TurnQueue {
        TurnQueue {
            queue: VecDeque::new(),
            element_set: HashSet::new(),
        }
    }
    
    pub fn push(&mut self, new_element: QueueElement) {
        if !self.element_set.contains(&new_element) {
            self.element_set.insert(new_element);
            self.queue.push_back(new_element);
        }
    }

    pub fn pop(&mut self) -> Option<QueueElement> {
        match self.queue.pop_front() {
            Some(elem) => {
                self.element_set.remove(&elem);
                return Some(elem);
            },
            None => None
        }
    }

    pub fn len(&self) -> usize {
        self.queue.len()
    }

    pub fn contains(&self, element: QueueElement) -> bool {
        self.element_set.contains(&element)
    }
}