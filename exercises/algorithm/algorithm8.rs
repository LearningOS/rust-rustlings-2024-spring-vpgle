/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/


#[derive(Debug)]
pub struct Queue<T> {
    elements: Vec<T>,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, value: T) {
        self.elements.push(value)
    }

    pub fn dequeue(&mut self) -> Result<T, &str> {
        if !self.elements.is_empty() {
            Ok(self.elements.remove(0usize))
        } else {
            Err("Queue is empty")
        }
    }

    pub fn peek(&self) -> Result<&T, &str> {
        match self.elements.first() {
            Some(value) => Ok(value),
            None => Err("Queue is empty"),
        }
    }

    pub fn size(&self) -> usize {
        self.elements.len()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

impl<T> Default for Queue<T> {
    fn default() -> Queue<T> {
        Queue {
            elements: Vec::new(),
        }
    }
}

pub struct myStack<T>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
            q1: Queue::new(),
            q2: Queue::new()
        }
    }

    pub fn push(&mut self, elem: T) {
        // To implement push, we want to enqueue the element into the non-empty queue
        // If both queues are empty, we'll enqueue into q1 by default
        if !self.q1.is_empty() {
            self.q1.enqueue(elem);
        } else {
            self.q2.enqueue(elem);
        }
    }

    pub fn pop(&mut self) -> Result<T, &str> {
        // To implement pop, we want to dequeue all elements from the non-empty queue
        // and enqueue them into the other queue until only one element is left
        // that last element is the one we want to pop

        let (src, dest) = if !self.q1.is_empty() {
            (&mut self.q1, &mut self.q2)
        } else if !self.q2.is_empty() {
            (&mut self.q2, &mut self.q1)
        } else {
            return Err("Stack is empty");
        };

        while src.size() > 1 {
            if let Some(val) = src.dequeue().ok() {
                dest.enqueue(val);
            }
        }

        src.dequeue().map_err(|_| "Stack is empty")
    }

    pub fn is_empty(&self) -> bool {
        self.q1.is_empty() && self.q2.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn test_queue(){
		let mut s = myStack::<i32>::new();
		assert_eq!(s.pop(), Err("Stack is empty"));
        s.push(1);
        s.push(2);
        s.push(3);
        assert_eq!(s.pop(), Ok(3));
        assert_eq!(s.pop(), Ok(2));
        s.push(4);
        s.push(5);
        assert_eq!(s.is_empty(), false);
        assert_eq!(s.pop(), Ok(5));
        assert_eq!(s.pop(), Ok(4));
        assert_eq!(s.pop(), Ok(1));
        assert_eq!(s.pop(), Err("Stack is empty"));
        assert_eq!(s.is_empty(), true);
	}
}