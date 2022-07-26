fn shell_sort<T: PartialOrd>(data: &mut [T]){

    let mut h = 1;

    // compute progressive increment
    while h < data.len() / 3 {
        h = 3 * h + 1; // 1, 4, 13, 40, ...
    }

    while h >= 1 {

        for i in 0..data.len(){

            let mut j = i;
            while j >= h && data[j-h] > data[j]{
                data.swap(j, j-h);
                j -= h;
            }  
        }
        h /= 3;
    }


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn already_sorted() {
        let mut data = [1, 2, 3, 4];
        shell_sort(&mut data);
        assert_eq!(&[1, 2, 3, 4], &data);
    }

    #[test]
    fn simple_slice() {
        let mut data = [4, 3, 2, 1];
        shell_sort(&mut data);
        assert_eq!(&[1, 2, 3, 4], &data);
    }

    #[test]
    fn char_slice() {
        let mut data = ["a", "c", "b", "f"];
        shell_sort(&mut data);
        assert_eq!(&["a", "b", "c", "f"], &data);
    }
}
