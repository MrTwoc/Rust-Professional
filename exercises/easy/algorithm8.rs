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
    // 用于存储元素的第一个队列
    q1:Queue<T>,
    // 用于存储元素的第二个队列
    q2:Queue<T>,
    // 用于选择当前使用的队列的布尔值
    select_queue:bool,

}
impl<T> myStack<T> {
    // 创建一个新的 myStack 实例
    pub fn new() -> Self {
        Self {
            // 创建两个新的空队列 q1 和 q2
            q1: Queue::<T>::new(),
            q2: Queue::<T>::new(),
            // 初始化 select_queue 为 true，表示当前选择的队列是 q1
            select_queue: true,
        }
    }
    // 将元素 elem 压入栈顶
    pub fn push(&mut self, elem: T) {
        // 如果 select_queue 为 true，则将元素压入 q1
        if self.select_queue{
            self.q1.enqueue(elem);
        // 如果 select_queue 为 false，则将元素压入 q2
        }else{
            self.q2.enqueue(elem);
        }
    }
    // 移除并返回栈顶元素
    pub fn pop(&mut self) -> Result<T, &str> {
        // 如果 select_queue 为 true，则从 q1 中移除并返回栈顶元素
        if self.select_queue {
            // 如果 q1 为空，则返回错误
            if self.q1.is_empty() {
                return Err("Stack is empty");
            }
            // 将 q1 中的元素依次出队并添加到 q2 中，直到 q1 中只剩下一个元素
            while self.q1.size() > 1 {
                let temp = self.q1.dequeue().unwrap();
                self.q2.enqueue(temp);
            }
            // 移除并返回 q1 中的最后一个元素
            let temp = self.q1.dequeue().unwrap();
            // 切换 select_queue 的值
            self.select_queue = !self.select_queue;
            // 返回移除的元素
            return Ok(temp);
        // 如果 select_queue 为 false，则从 q2 中移除并返回栈顶元素
        } else {
            // 如果 q2 为空，则返回错误
            if self.q2.is_empty() {
                return Err("Stack is empty");
            }
            // 将 q2 中的元素依次出队并添加到 q1 中，直到 q2 中只剩下一个元素
            while self.q2.size() > 1 {
                let temp = self.q2.dequeue().unwrap();
                self.q1.enqueue(temp);
            }
            // 移除并返回 q2 中的最后一个元素
            let temp = self.q2.dequeue().unwrap();
            // 切换 select_queue 的值
            self.select_queue = !self.select_queue;
            // 返回移除的元素
            return Ok(temp);
        }
        // 如果栈为空，则返回错误
        Err("Stack is empty")
    }
    // 检查栈是否为空
    pub fn is_empty(&self) -> bool {
        // 如果 select_queue 为 true，则检查 q1 是否为空
        if self.select_queue {
            return self.q1.is_empty();
        // 如果 select_queue 为 false，则检查 q2 是否为空
        } else {
            return self.q2.is_empty();
        }
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