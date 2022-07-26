//!
//! It is an eager approach to solve the dynamic connectivity problem.
//! It uses as base data structure a Vec of integer of size N where p and q, which are two objects, are connected iff they have the same id. This allows a quick implementation of the find operation where the only things to do is to check if they have the same id.
//! 
//! ```rust
//!     # use union_finding::quickfind::UT;
//!     let ut = UT::new(3);
//!     assert_eq!(false, ut.connected(0, 1));
//! ```
//! 
//! A more difficult operation is union, in order to merge components containing two given objects p and q, it's necessary to change all entries whose id equals id\[p\] to id\[q\]. This can be also inefficient when there are a lot of values that can change.
//!
//! ```rust
//!     # use union_finding::quickfind::UT;
//!     let mut ut = UT::new(4);
//!     ut.union(0, 1);
//!     assert_eq!(true, ut.connected(0, 1));
//! ```

/// The basic struct
pub struct UT {
    id: Vec<u32>
}

impl UT{

    /// Return a new Quick Find data structure instance with capacity n
    pub fn new(n: u32) -> UT{

        let mut r: Vec<u32> = Vec::with_capacity(n as usize);

        for i in 0..n{
            r.push(i);
        }

        UT{
            id: r
        }

    }

    /// check if two components are connected
    pub fn connected(&self, p: usize, q: usize) -> bool{
        &self.id[p] == &self.id[q]
    }

    /// connect two components
    pub fn union(&mut self, p: usize, q: usize){
        // inefficient. Considering M operations on N items this requires O(NM)
        let pid = self.id[p];
        let qid = self.id[q];
        
        for i in 0..self.id.len(){
            if self.id[i] == pid {
                self.id[i] = qid;
            }
        }
    }

}

#[cfg(test)]
mod test{

    use super::UT;

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