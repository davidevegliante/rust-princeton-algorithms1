//!
//! This is a lazy approach. We think of the array as a set of trees. Each entry in the array contains a reference to its parent. So every compoment has a root.
//! 
//! ```rust
//!     # use union_finding::quickunion::UT;
//!     let ut = UT::new(3);
//!     assert_eq!(false, ut.connected(0, 1));
//! ```
//! 
//! This approach is faster than quick find but still inefficient considering that the tree could get flat
//!  
//! ```rust
//!     # use union_finding::quickunion::UT;
//!     let mut ut = UT::new(4);
//!     ut.union(0, 1);
//!     assert_eq!(true, ut.connected(0, 1));
//! ```
pub struct UT {
    id: Vec<u32>
}

impl UT{

    /// Return a new Quick Union data structure instance with capacity n
    pub fn new(n: u32) -> UT{
        let mut r: Vec<u32> = Vec::with_capacity(n as usize);
        for i in 0..n{
            r.push(i);
        }

        UT{
            id: r
        }
    }


    /// Navigate the tree until it find the root of the input component 
    pub fn root(&self, mut i: usize) -> usize{
        // tree can get tall and this is the defect of this approach
        // root operation could involve an access to all the element of the array

        // we find the root when the element we're checking points to itself
        while self.id[i as usize] != i as u32{
            i = self.id[i] as usize;
        }

        self.id[i as usize] as usize
    }

    /// Check if two components are connected looking if they're part of the same tree
    pub fn connected(&self, p: usize, q: usize)-> bool{
        self.root(p) == self.root(q)
    }

    /// Merge two components making p root points to q root
    pub fn union(&mut self, p: usize, q: usize){
        let pi = self.root(p);
        let qi = self.root(q);
        self.id[pi] = qi as u32;
    }

}

#[cfg(test)]
mod test{

    use super::*;

    #[test]
    pub fn simple_init(){

        let ut = UT::new(3);
        assert_eq!(ut.id, [0,1,2])

    }

    #[test]
    pub fn connected_shoud_fail(){
        let ut = UT::new(10);

        assert_eq!(false, ut.connected(0, 1));
        assert_eq!(false, ut.connected(0, 2));
        assert_eq!(false, ut.connected(3, 2));
    }

    pub fn connected(){
        let mut ut = UT::new(4);
        ut.union(0, 1);
        ut.union(2, 3);
        
        assert_eq!(true, ut.connected(0, 1));
        assert_eq!(true, ut.connected(2, 3));
        assert_eq!(false, ut.connected(0, 3));
    
    }

}