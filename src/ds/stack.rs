// Define a basic Node structure for the elements in the stack
#[derive(Debug)]
struct Node<T> {
    data: T,
    next: Option<Box<Node<T>>>,
}

// Define the Stack structure
#[derive(Debug)]
pub struct Stack<T> {
    top: Option<Box<Node<T>>>,
}

// Implementation of the Stack methods
impl<T> Stack<T> {
    // Create a new empty stack
    pub fn new() -> Self {
        Stack { top: None }
    }

    // Check if the stack is empty
    pub fn is_empty(&self) -> bool {
        self.top.is_none()
    }

    // Push a new element onto the stack
    pub fn push(&mut self, data: T) {
        let new_node = Box::new(Node {
            data,
            next: self.top.take(),
        });
        self.top = Some(new_node);
    }

    // Pop the top element from the stack
    pub fn pop(&mut self) -> Option<T> {
        self.top.take().map(|node| {
            self.top = node.next;
            node.data
        })
    }

    // Peek at the top element without removing it
    pub fn peek(&self) -> Option<&T> {
        self.top.as_ref().map(|node| &node.data)
    }
}

// Example usage
fn main() {
    let mut stack = Stack::new();

    // Push elements onto the stack
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // Peek at the top element
    println!("Top element: {:?}", stack.peek()); // Output: Some(3)

    // Pop elements from the stack
    println!("Popped: {:?}", stack.pop()); // Output: Some(3)
    println!("Popped: {:?}", stack.pop()); // Output: Some(2)

    // Check if the stack is empty
    println!("Is empty: {:?}", stack.is_empty()); // Output: false

    // Pop the last element
    println!("Popped: {:?}", stack.pop()); // Output: Some(1)

    // Check if the stack is empty after popping all elements
    println!("Is empty: {:?}", stack.is_empty()); // Output: true
}
