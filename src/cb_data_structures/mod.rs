/// A Linked List structure
#[derive(Debug, Copy, Clone)]
pub struct CbLinkedList<T> {
    value: T,
    next: Option<T>,
}

impl<T> CbLinkedList<T> {
    /// Create a new LinkedList with the given value
    pub fn new(value: T) -> Self {
        return Self {
            value: value,
            next: None,
        };
    }

    /// Get the value for the current node
    pub fn value(&self) -> &T {
        return &self.value;
    }

    /// Add a new next node
    pub fn add_next(&mut self, next: T) {
        self.next = Some(next);
    }

    /// Remove the next node
    pub fn remove_next(&mut self) {
        self.next = None;
    }
}
