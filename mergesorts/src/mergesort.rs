use crate::utils::merge;

fn merge_sort<T: Ord + Clone>(data: &mut [T]){

    // create auxiliary array once 
    let mut aux = data.to_vec();

    // call recursive_routine
    recursive_routine(data, &mut aux, 0, data.len()-1);

}

fn recursive_routine<T: Ord + Clone>(slice: &mut [T], aux: &mut [T], left: usize, right: usize){

    // a possible optimization could be done here!
    // it's proven that merge sort is not efficient for small array
    // we could use a faster sorting algorithm when the number of 
    // elements in the sub-array is less than eg. 7

    if left < right {
        let mid = left + (right - left) / 2;
        recursive_routine(slice, aux, left, mid);
        recursive_routine(slice, aux, mid+1, right);

        // if the last element of the 1st half is smaller
        // than the first of the 2nd, the array is already sorted
        if slice[mid] < slice[mid+1]{
            return
        }

        // otherwise we need to merge them
        merge(slice, aux, left, mid, right);
    }
}



#[cfg(test)]
mod tests{

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
