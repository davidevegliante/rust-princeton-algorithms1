use crate::partition::*;
use rand::seq::SliceRandom;


fn select<T: Ord + Clone>(data: &mut [T], k: usize) -> Result<T, &'static str>{
    if k >= data.len(){
        Err("Invalid Argument")

    } else{
        let mut lo = 0;
        let mut hi = data.len()-1;

        let mut rng = rand::thread_rng();
        data.shuffle(&mut rng);
        let mut j = lo;

        while hi > lo{

            j = standard_partition(data, lo, hi);
            if j < k {
                lo = j + 1;

            } else if j > k{
                hi = j - 1;

            }else{ return Ok(data[j].clone()) }

        }
        Ok(data[lo].clone())
    }

}

#[cfg(test)]
mod tests{
    use super::*;

   #[test]
    fn illegal_k(){
        let mut data = [6,5,3,7,1,10];
        let res = select(&mut data, 15);
        assert_eq!(res.is_err(), true);
    }

    #[test]
    fn simple_test0(){
        let mut data = [6,5,3,7,1,10];
        let res = select(&mut data, 0);
        assert_eq!(res.unwrap(), 1);
    }

    #[test]
    fn simple_test1(){
        let mut data = [6,5,3,7,1,10];
        let res = select(&mut data, 1);
        assert_eq!(res.unwrap(), 3);
    }

    #[test]
    fn simple_test2(){
        let mut data = [6,5,3,7,1,10];
        let res = select(&mut data, 2);
        assert_eq!(res.unwrap(), 5);
    }


    #[test]
    fn test0(){
        let mut data = [6,5,30,7,15,10,1,2,9,9,10];
        let res = select(&mut data, 10);
        assert_eq!(res.unwrap(), 30);
    }


    #[test]
    fn test1(){
        let mut data = [6,5,30,7,15,10,1,2,9,9,10];
        let res = select(&mut data, 9);
        assert_eq!(res.unwrap(), 15);
    }

    #[test]
    fn test2(){
        let mut data = [6,5,30,7,15,10,1,2,9,9,10];
        let res = select(&mut data, 0);
        assert_eq!(res.unwrap(), 1);
    }

    #[test]
    fn test_char0(){
        let mut data = ["a", "c", "z", "h", "g", "f", "f", "n", "m"];
        let res = select(&mut data, 4);
        assert_eq!(res.unwrap(), "g");
    }

    #[test]
    fn test_char1(){
        let mut data = ["a", "c", "z", "h", "g", "f", "f", "n", "m"];
        let res = select(&mut data, 3);
        assert_eq!(res.unwrap(), "f");
    }

}
