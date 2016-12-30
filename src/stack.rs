#[derive(Debug)] // TODO remove debug
pub struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    #[inline]
    pub fn new() -> Stack<T> {
        Stack(Vec::new())
    }

    #[inline]
    pub fn with_capacity(capacity: usize) -> Stack<T> {
        Stack(Vec::with_capacity(capacity))
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        self.0.push(value)
    }

    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }
}
