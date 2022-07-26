pub fn standard_partition<T: Ord>(data: &mut [T], lo: usize, hi: usize) -> usize{
    
    /*
        Standard partitioning algorithm that uses two pointers.
    */

    // var initialization
    let mut i = lo+1;
    let mut j = hi;

    loop{

        // find item on left to swa
        while i <= hi && data[i] < data[lo]{
            if i == hi{
                break;
            }
            i+=1;
        }

        // find item on right to swap
        while data[j] > data[lo]{
            if j == lo{
                break;
            }
            j-=1;
        }

        // break when the two pointers cross
        if i >= j{
            break;
        }

        data.swap(i, j);
    }

    // swap with partition item
    data.swap(lo, j);
    return j;
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn standard_simple_test(){
        let mut data = [0,1,2,3,4,5];
        let hi = data.len() - 1;
        let j = standard_partition(&mut data, 0, hi);
        assert_eq!(0, j)
    }

    #[test]
    fn standard_reverse_simple_test() {
        let mut data = [5, 4, 3, 2, 1];
        let hi = data.len() - 1;
        let j = standard_partition(&mut data, 0, hi);
        assert_eq!(4, j);
        assert_eq!(&data, &[1, 4, 3, 2, 5]);
    }

}