use std::rc::Rc;
use std::cell::RefCell;

struct Node<T>{
    elem: T,
    next: Link<T>
}

type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Queue<T>{
    head: Link<T>,
    tail: Link<T>
}

impl<T> Queue<T>{

    pub fn new() -> Queue<T>{
        Queue{head: None, tail: None}
    }

    pub fn enqueue(&mut self, elem: T){
        match self.tail.take(){
            // tail equal to None, we can assume head equal to None too
            None => {
                // Since the queue is empty, create a new node and use it as head and tail
                let new_node = Rc::new(RefCell::new(Node{elem: elem, next: None}));
                // here it comes useful the Rc. We can now have multiple references to a node
                self.head = Some(new_node.clone()); 
                self.tail = Some(new_node.clone()); 
            },
            Some(old_tail) =>{
                // Not empty queue. Insert a new node at the end of the struct
                // Create a new tail with an empty next pointer,
                // make the actual tail point to the newly created node
                // set the new node as tail
                let new_tail = Some(Rc::new(RefCell::new(Node{elem: elem, next: None})));
                // here we use interior mutabolity to change the value of the next pointer
                old_tail.borrow_mut().next = new_tail.clone(); 
                self.tail = new_tail; 
            }
        }
    }

    pub fn dequeue(&mut self) -> Option<T>{
        // take the head out 
        self.head.take().map(|old_head| {
            // match the value of the actual head's pointer
            match old_head.borrow_mut().next.take() {
                // the head has a pointer to another element
                // simply change the head and make it point to the next node
                Some(new_head) => {
                    self.head = Some(new_head);
                }
                // actual head with a None pointer => head == tail
                // in this case we need to take and remove the rc to the node
                // otherwise the next try_unwrap will fail
                None => {
                    self.tail.take();
                }
            }
            // here try to unwrap the rc, transform the result into an option
            // unwrap the option and then consume the RefCell. Now we can return the elem 
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().elem
        })
    }

    pub fn is_empty(&self) -> bool{
        match self.head{
            None => true,
            Some(_) => false
        }
    }

}

impl<T> Iterator for Queue<T>{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item>{
        self.dequeue()
    }

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]    
    fn init() {
        let simple_queue = Queue::<u32>::new();
        assert_eq!(true, simple_queue.head.is_none());
        assert_eq!(true, simple_queue.is_empty())
    }

    
    #[test]    
    fn empty_dequeue() {
        let mut simple_queue = Queue::<u32>::new();
        assert_eq!(None, simple_queue.dequeue())
    }
    
    #[test]    
    fn iteration() {
        let mut simple_queue = Queue::<u32>::new();
        simple_queue.enqueue(1);
        simple_queue.enqueue(2);
        simple_queue.enqueue(3);
        simple_queue.enqueue(4);

        assert_eq!(Some(1), simple_queue.next());
        assert_eq!(Some(2), simple_queue.next());
        assert_eq!(Some(3), simple_queue.next());
        assert_eq!(Some(4), simple_queue.next());
        assert_eq!(None, simple_queue.next());
    }

    #[test]    
    fn first_enqueue_and_dequeue() {
        let mut simple_stack = Queue::<u32>::new();
        simple_stack.enqueue(5);
        assert_eq!(Some(5), simple_stack.dequeue());
        assert_eq!(None, simple_stack.dequeue())
    }
    

    
    #[test]    
    fn some_enqueue_and_dequeue(){
        let mut simple_stack = Queue::<u32>::new();
        simple_stack.enqueue(5);
        simple_stack.enqueue(6);
        simple_stack.enqueue(7);
        assert_eq!(Some(5), simple_stack.dequeue());
        assert_eq!(Some(6), simple_stack.dequeue());
        assert_eq!(Some(7), simple_stack.dequeue());
        assert_eq!(None, simple_stack.dequeue())
    } 
}
