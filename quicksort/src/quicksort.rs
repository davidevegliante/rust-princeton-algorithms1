use crate::partition::*;
extern crate rand;
use rand::seq::SliceRandom;
use crate::djistra::*;

fn quicksort<T: Ord>(data: &mut [T]){
    /* pratical improvements:
        - cut-off to insertion for small sub-arrays
        - median of sample
     */

    let mut rng = rand::thread_rng();
    data.shuffle(&mut rng);
    recursive_step(data, 0, data.len()-1);
}

fn recursive_step<T: Ord>(data: &mut [T], lo: usize, hi: usize){

    if hi <= lo{
        return
    }

    let mut j = standard_partition(data, lo, hi);
    if j > 0{
        recursive_step(data, lo, j-1);
    }
    recursive_step(data, j+1, hi);

}

fn djistra_quicksort<T: Ord>(data: &mut [T]){
    recursive_step(data, 0, data.len() - 1);
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn already_sorted_standard(){
        let mut data = [0,1,2,3,4,5,6,7,8,9,10];
        let ordered = data.clone();
        quicksort(&mut data);
        assert_eq!(data, ordered);
    }

    #[test]
    fn already_sorted_djistra(){
        let mut data = [0,1,2,3,4,5,6,7,8,9,10];
        let ordered = data.clone();
        djistra_quicksort(&mut data);
        assert_eq!(data, ordered);
    }

    #[test]
    fn reverse_order_standard(){
        let mut data = [10,9,8,7,6,5,4,3,2,1,0];
        quicksort(&mut data);
        assert_eq!(data, [0,1,2,3,4,5,6,7,8,9,10]);
    }

    #[test]
    fn reverse_order_djistra(){
        let mut data = [10,9,8,7,6,5,4,3,2,1,0];
        quicksort(&mut data);
        assert_eq!(data, [0,1,2,3,4,5,6,7,8,9,10]);
    }

    #[test]
    fn simple_test_standard(){
        let mut things = vec![4,2,5,7,8];
        quicksort(&mut things);
        assert_eq!(things, [2,4,5,7,8]);
    }

    #[test]
    fn simple_test_djistra(){
        let mut things = vec![4,2,5,7,8];
        quicksort(&mut things);
        assert_eq!(things, [2,4,5,7,8]);
    }
    
    #[test]
    fn simple_test_two_standard(){
        let mut things = vec![9,8,9,5,1,3,2,10];
        quicksort(&mut things);
        assert_eq!(things, [1,2,3,5,8,9,9,10]);
    }

    #[test]
    fn simple_test_two_djistra(){
        let mut things = vec![9,8,9,5,1,3,2,10];
        quicksort(&mut things);
        assert_eq!(things, [1,2,3,5,8,9,9,10]);
    }
    
    #[test]
    fn another_test_standard(){
        let mut things = vec![12, 16 ,8, 15, 10, 6, 3, 9, 5];
        quicksort(&mut things);
        assert_eq!(things, [3, 5, 6, 8, 9, 10, 12, 15, 16]);
    }

    #[test]
    fn another_test_djistra(){
        let mut things = vec![12, 16 ,8, 15, 10, 6, 3, 9, 5];
        quicksort(&mut things);
        assert_eq!(things, [3, 5, 6, 8, 9, 10, 12, 15, 16]);
    }

    #[test]
    fn duplicates_standard(){
        let mut things = vec![12, 12 ,8, 15, 10, 8, 3, 9, 5];
        quicksort(&mut things);
        assert_eq!(things, [3, 5, 8, 8, 9, 10, 12, 12, 15]);
    }

    #[test]
    fn duplicates_djistra(){
        let mut things = vec![12, 12 ,8, 15, 10, 8, 3, 9, 5];
        quicksort(&mut things);
        assert_eq!(things, [3, 5, 8, 8, 9, 10, 12, 12, 15]);
    }

    #[test]
    fn on_string_standard(){
        let mut things = vec!["z", "b", "a", "g", "n"];
        quicksort(&mut things);
        assert_eq!(things, ["a", "b", "g", "n", "z"]);
    }

    #[test]
    fn on_string_djistra(){
        let mut things = vec!["z", "b", "a", "g", "n"];
        quicksort(&mut things);
        assert_eq!(things, ["a", "b", "g", "n", "z"]);
    }
}