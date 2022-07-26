struct Stack<T: Sized>{
    data: [Option<T>; 1],
    n: usize,
    size: usize
}

impl<T> Stack<T>{
    pub fn new() -> Stack<T>{
        Stack{
            n: 0,
            size: 1,
            data: [None]
        }
    }

    pub fn is_empty(&self) -> bool {
        if self.n > 0{
            false
        }else{
            true
        }
        
    }

    fn double(&mut self){
        let new_size = self.size * 2;
        let new_data: [Option<T>; new_size] = [None, new_size];
    }

    pub fn push(&mut self, elem: T){

        if self.n == self.size{
            // the stack is full. Double the size
            self.data.reserve(self.n * 2);
        }
        self.data[self.n] = elem;
        self.n +=1;
    }

    pub fn pop(&mut self) -> Option<T>{
        let capacity = self.data.capacity();

        // if the array is half full, halve it to 1/4 of its capacity
        if self.n > 0 && capacity / 4 == self.n{

            let mut shrinked = Vec::<T>::with_capacity(capacity / 2 as usize);
            for i in 0..self.data.len(){
                shrinked.push(self.data.swap_remove(0));
            }
            self.n -=1;
            self.data.pop()

        }else if self.n > 0 {
            self.data.pop()
        }else{
            None
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
        assert_eq!(true, simple_stack.is_empty())
    }

    #[test]
    fn test_capacity(){
        let mut simple_stack = Stack::<u32>::new();
        simple_stack.push(1);
        assert_eq!(1, simple_stack.data.capacity());
        simple_stack.push(2);
        assert_eq!(2, simple_stack.data.capacity());
        simple_stack.push(3);
        assert_eq!(4, simple_stack.data.capacity());

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

    fn iterator(){
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
}
