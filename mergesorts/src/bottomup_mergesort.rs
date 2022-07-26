use std::cmp;
use crate::utils::merge;

/**
 * Iterative version of merge-sort
 */
fn merge_sort<T: Ord + Clone>(data: &mut [T]){

    // create auxiliary array once 
    let mut aux = data.to_vec();

    let mut sz = 1;
    
    // iterate over sub-arrays 2, 4, 8...
    while sz < data.len(){

        let mut low = 0;
        while low < data.len() - sz{
            
            merge(data, &mut aux, low, low+sz-1, cmp::min(low+sz+sz-1, data.len()-1));
            low += sz + sz;

        }

        sz = sz+sz;
    }

}

#[cfg(test)]
mod test{

    use super::*;
 
    #[test]
    fn simple_test(){
        let mut things = vec![4,2,5,7,8];
        merge_sort(&mut things);
        assert_eq!(things, [2,4,5,7,8]);
    }
    
    #[test]
    fn simple_test_two(){
        let mut things = vec![9,8,9,5,1,3,2,10];
        merge_sort(&mut things);
        assert_eq!(things, [1,2,3,5,8,9,9,10]);
    }
    
    #[test]
    fn another_test(){
        let mut things = vec![12, 16 ,8, 15, 10, 6, 3, 9, 5];
        merge_sort(&mut things);
        assert_eq!(things, [3, 5, 6, 8, 9, 10, 12, 15, 16]);
    }
    
    #[test]
    fn on_string(){
        let mut things = vec!["z", "b", "a", "g", "n"];
        merge_sort(&mut things);
        assert_eq!(things, ["a", "b", "g", "n", "z"]);
    }
}