/// An abstract data type that is a collection of elements arranged in a LIFO (Last In, First Out) structure.
#[derive(Debug, Clone, PartialEq)]
pub struct Stack<T> {
    arr: Vec<T>,
    pub size: usize,
    pub is_empty: bool,
}

impl<T> Stack<T> {
    /// Create a new stack.
    pub fn new() -> Self {
        Stack {
            arr: vec![],
            size: 0,
            is_empty: true,
        }
    }
    /// Push a piece of data onto the stack.
    pub fn push(&mut self, data: T) {
        self.arr.push(data);
        self.size += 1;
        self.is_empty = self.is_empty && false;
    }
    /// Removes a piece of data from the stack and returns it.
    pub fn pop(&mut self) -> Option<T> {
        if self.size > 0 {
            self.size -= 1;
            self.is_empty = self.size == 0;
        }
        self.arr.pop()
    }
    /// Peeks at the element at the top of the stack.
    pub fn top(&self) -> Option<&T> {
        self.arr.last()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_tests_stack_pop() {
        let mut stack: Stack<i32> = Stack::new();
        stack.push(78);
        stack.push(34);
        stack.pop();
        assert_eq!(stack.top().unwrap(), &78);
    }
    #[test]
    fn it_test_stack_emptiness() {
        let mut stack: Stack<i32> = Stack::new();
        stack.push(56);
        stack.pop();
        assert!(stack.is_empty);
    }
}
