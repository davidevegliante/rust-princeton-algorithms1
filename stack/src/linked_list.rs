
// every node owns an element and a link to another node
// The next link is an option to Boxed Node
pub struct Node<T>{
    elem: T,
    next: Link<T>
}

type Link<T> = Option<Box<Node<T>>>;


// main struct
pub struct Stack<T>{
    head: Link<T>
}

impl<T> Stack<T>{

    pub fn new() -> Stack<T>{
        Stack{  
            head: None
        }
    }

    pub fn push(&mut self, elem: T){
        // every push operation takes constant time
        // we need to check the value of head
        // since it's an option
        self.head = match self.head{
            // we init the stack with a new boxed node
            None => { Some(Box::new(Node{elem: elem, next: None})) }
            // stack already initialized
            // create the new head and make it point to
            // the old one
            Some(_) => {
                let old_head = self.head.take(); // take the option and replace with None
                Some(Box::new(Node{elem: elem, next: old_head}))
            }
        }
    }

    pub fn pop(&mut self)-> Option<T>{
        // constant amount of time
        match self.head{
            // if stack is empty we return None
            None => None,
            // otherwise we take and safely unwrap the old head
            // make the old head the new head and return the elem
            Some(_) => {
                let old_head = self.head.take().unwrap();
                self.head = old_head.next;
                Some(old_head.elem)
            }
        }
    }

    pub fn is_empty(&self) -> bool{
        match self.head{
            None => true,
            Some(_) => false
        }
    }

}

impl<T> Iterator for Stack<T>{
    type Item = T;

    fn next(&mut self) -> Option<T>{
        self.pop()
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]    
    fn init() {
        let simple_stack = Stack::<u32>::new();
        assert_eq!(true, simple_stack.head.is_none());
        assert_eq!(true, simple_stack.is_empty())
    }

    #[test]    
    fn empty_pop() {
        let mut simple_stack = Stack::<u32>::new();
        assert_eq!(None, simple_stack.pop())
    }

    #[test]    
    fn first_push_and_pop() {
        let mut simple_stack = Stack::<u32>::new();
        simple_stack.push(5);
        assert_eq!(Some(5), simple_stack.pop());
        assert_eq!(None, simple_stack.pop())
    }

    #[test]    
    fn some_push(){
        let mut simple_stack = Stack::<u32>::new();
        simple_stack.push(5);
        simple_stack.push(6);
        simple_stack.push(7);
        assert_eq!(Some(7), simple_stack.pop());
        assert_eq!(Some(6), simple_stack.pop());
        assert_eq!(Some(5), simple_stack.pop());
        assert_eq!(None, simple_stack.pop())
    }

    #[test]    
    fn iteration() {
        let mut simple_stack = Stack::<u32>::new();
        simple_stack.push(1);
        simple_stack.push(2);
        simple_stack.push(3);
        simple_stack.push(4);

        assert_eq!(Some(4), simple_stack.next());
        assert_eq!(Some(3), simple_stack.next());
        assert_eq!(Some(2), simple_stack.next());
        assert_eq!(Some(1), simple_stack.next());
        assert_eq!(None, simple_stack.next());
    }
}
