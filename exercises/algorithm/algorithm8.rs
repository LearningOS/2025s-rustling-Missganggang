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
            //TODO
            q1:Queue::<T>::new(),
            q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        // 将元素添加到非空的队列中，如果两个队列都为空，则添加到 q1
        if self.q1.is_empty() {
            self.q2.enqueue(elem);
        } else {
            self.q1.enqueue(elem);
        }
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        // 将非空队列中的元素依次出队并加入到另一个队列中，直到剩下一个元素，该元素即为栈顶元素
        if self.q1.is_empty() && self.q2.is_empty() {
            return Err("Stack is empty");
        }

        let (mut source, mut target) = if self.q1.is_empty() {
            (&mut self.q2, &mut self.q1)
        } else {
            (&mut self.q1, &mut self.q2)
        };

        while source.size() > 1 {
            target.enqueue(source.dequeue().unwrap());
        }

        source.dequeue()
    }
    pub fn is_empty(&self) -> bool {
        // 当两个队列都为空时，栈为空
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