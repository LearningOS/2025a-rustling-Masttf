/*
	queue
	This question requires you to use queues to implement the functionality of the stac
*/
// I AM NOT DONE

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

pub struct MyStack<T>
{
	//TODO
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> MyStack<T> {
    pub fn new() -> Self {
        Self {
			//TODO
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        // 总是向 q1 中添加新元素
        self.q1.enqueue(elem);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        // 如果 q1 为空，则栈为空
        if self.q1.is_empty() {
            return Err("Stack is empty");
        }

        // 将 q1 中除最后一个元素外的所有元素移到 q2
        while self.q1.size() > 1 {
            if let Ok(elem) = self.q1.dequeue() {
                self.q2.enqueue(elem);
            }
        }

        // q1 中剩下的最后一个元素就是栈顶元素
        // 分离借用，先取出结果
        let result = match self.q1.dequeue() {
            Ok(value) => Ok(value),
            Err(_) => Err("Stack is empty"), // 这里不应该发生，因为前面检查过了
        };

        // 交换 q1 和 q2，使 q1 始终是主队列
        std::mem::swap(&mut self.q1, &mut self.q2);

        result
    }
    pub fn is_empty(&self) -> bool {
        // 栈为空当且仅当 q1 为空（因为所有元素都在 q1 中）
        self.q1.is_empty()
    }
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_queue(){
		let mut s = MyStack::<i32>::new();
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
