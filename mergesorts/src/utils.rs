pub fn merge<T: Ord + Clone>(data: &mut [T], aux: &mut [T], left: usize, mid: usize, right: usize){
    
    // take aux array from outside
    // we're avoiding to create it every time we need to perform a merge
    // init aux array with the correct values
    // another improvement that could be done is eliminate this copy
    // by switching the role of the input and auxiliary array in each rec call
    for i in left..right+1{
        aux[i] = data[i].clone();
    }

    let mut i = left;
    let mut j = mid+1;

    for k in left..right+1{
        
        // if we already copied all the values at left, simply copy right's
        if i > mid{
            data[k] = aux[j].clone();
            j+=1;

        // same for right part
        }else if j > right{
            data[k] = aux[i].clone();
            i+=1;

        }else if aux[i] < aux[j]{
            data[k] = aux[i].clone();
            i+=1;
        }else{
            data[k] = aux[j].clone();
            j+=1;
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;


    #[test]
    fn two_elem(){
        let mut things = vec![2,1];
        let mut aux = things.to_vec();
        let len = things.len();
        merge(&mut things, &mut aux, 0, 0, len-1);
        assert_eq!(things, [1,2]); 
    }

    #[test]
    fn four_elem(){
        let mut things = vec![1,2,0,4];
        let mut aux = things.to_vec();
        let len = things.len();
        merge(&mut things, &mut aux, 0, 1, len-1);
        assert_eq!(things, [0,1,2,4]);
    }

    #[test]
    fn eight_elem(){
        let mut things = vec![0,2,4,5,1,3,6,7];
        let mut aux = things.to_vec();
        let len = things.len();
        merge(&mut things, &mut aux, 0, 3, len-1);
        assert_eq!(things, [0,1,2,3,4,5,6,7]);
    }

}