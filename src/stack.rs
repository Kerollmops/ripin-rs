#[derive(Debug)] // TODO remove debug
pub struct Stack<T>(Vec<T>);

impl<T> Stack<T> {
    /// Creates an empty VecDeque.
    ///
    /// # Examples
    ///
    /// ```
    /// use ripin::Stack;
    ///
    /// let mut stack: Stack<i32> = Stack::new();
    /// ```
    #[inline]
    pub fn new() -> Stack<T> {
        Stack(Vec::new())
    }

    /// Creates an empty VecDeque with space for at least n elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use ripin::Stack;
    ///
    /// let mut stack: Stack<i32> = Stack::with_capacity(10);
    /// ```
    #[inline]
    pub fn with_capacity(capacity: usize) -> Stack<T> {
        Stack(Vec::with_capacity(capacity))
    }

    /// Returns the number of elements in the set.
    ///
    /// # Examples
    ///
    /// ```
    /// use ripin::Stack;
    ///
    /// let mut stack = Stack::new();
    /// stack.push(3);
    /// assert_eq!(stack.len(), 1);
    /// ```
    #[inline]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Returns true if the set contains no elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use ripin::Stack;
    ///
    /// let mut stack = Stack::new();
    /// assert_eq!(stack.is_empty(), true);
    /// stack.push(3);
    /// assert_eq!(stack.is_empty(), false);
    /// ```
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    /// Appends an element to the back of the stack.
    ///
    /// # Panics
    ///
    /// Panics if the number of elements in the stack overflows a usize.
    ///
    /// # Examples
    ///
    /// ```
    /// use ripin::Stack;
    ///
    /// let mut stack = Stack::new();
    /// stack.push(3);
    /// assert_eq!(stack.pop(), Some(3));
    /// assert_eq!(stack.pop(), None);
    /// ```
    #[inline]
    pub fn push(&mut self, value: T) {
        self.0.push(value)
    }

    /// Removes the last element from the stack and returns it,
    /// or `None` if it is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use ripin::Stack;
    ///
    /// let mut stack = Stack::new();
    /// stack.push(3);
    /// stack.push(2);
    /// assert_eq!(stack.pop(), Some(2));
    /// assert_eq!(stack.pop(), Some(3));
    /// assert_eq!(stack.pop(), None);
    /// ```
    #[inline]
    pub fn pop(&mut self) -> Option<T> {
        self.0.pop()
    }
}
