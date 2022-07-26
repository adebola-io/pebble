#[derive(Debug, Clone)]
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
