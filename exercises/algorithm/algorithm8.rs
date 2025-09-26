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
	//使用两个队列实现栈
	q1:Queue<T>,
	q2:Queue<T>
}
impl<T> myStack<T> {
    pub fn new() -> Self {
        Self {
			//初始化两个空队列
			q1:Queue::<T>::new(),
			q2:Queue::<T>::new()
        }
    }
    pub fn push(&mut self, elem: T) {
        // 入栈：将元素放入主队列 q1
        self.q1.enqueue(elem);
    }
    pub fn pop(&mut self) -> Result<T, &str> {
        // 出栈：将q1 中除了最后一个元素外的所有元素依次移动到 q2，
        // 然后交换 q1和q2，再从q2（原q1）取出最后一个元素。
        if self.q1.is_empty() {
            return Err("Stack is empty");
        }

        while self.q1.size() > 1 {
            // 这里unwrap 是安全的：size()>1时dequeue 必然成功
            let v = self.q1.dequeue().unwrap();
            self.q2.enqueue(v);
        }

        // 先交换队列使q2 成为含最后一个元素的队列（原q1）
        std::mem::swap(&mut self.q1, &mut self.q2);

        // 现在q2（原q1）包含仅剩的最后一个元素，直接dequeue即为栈顶
        let res = self.q2.dequeue();

        // 注意 此时q1已经是主队列（含剩余元素），q2已为空或已被消费
        res
    }
    pub fn is_empty(&self) -> bool {
		// 栈为空当且仅当主队列q1 为空
        self.q1.is_empty()
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