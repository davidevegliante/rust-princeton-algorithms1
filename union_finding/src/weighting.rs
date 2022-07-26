//!
//! It applies some improvements to quickunion.
//! Avoid tall trees;
//! Keep track of size of each tree (number of objects) using an additional data structure
//! Balance by linking root of smaller tree to root of larger tree
//! 

pub struct UT {
    id: Vec<u32>,
    weights: Vec<u32>
}

impl UT{
    pub fn new(n: u32) -> UT{

        let mut r: Vec<u32> = Vec::with_capacity(n as usize);
        let mut w: Vec<u32> = Vec::with_capacity(n as usize);
        for i in 0..n{
            r.push(i);
            w.push(1)
        }

        UT{
            id: r,
            weights: w
        }

    }

    pub fn root(&mut self, mut i: usize)->usize{

        while self.id[i as usize] != i as u32{
            /*
                A solution to make this op faster is path compression.
                With this one-line trick we make every other node in path
                points to its grandparent (halving path length).
                
                Alternatively, we might add a second loop to point each visited node points to its root.
            */
            self.id[i] = self.id[self.id[i] as usize];
            i = self.id[i] as usize;
        }

        self.id[i] as usize
    }


    pub fn connected(&mut self, p: usize, q: usize)-> bool{
        self.root(p) == self.root(q)
    }


    pub fn union(&mut self, p: usize, q: usize){
        // this is smart. Depth of any nodes can be at most logN
        let pi = self.root(p);
        let qi = self.root(q);

        if pi == qi {return}
        
        if self.weights[pi] > self.weights[qi]{
            self.id[qi] = pi as u32;
            self.weights[pi] += self.weights[qi];
        }else{
            self.id[pi] = qi as u32;
            self.weights[qi] += self.weights[pi];    
        }

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
        let mut ut = UT::new(10);

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