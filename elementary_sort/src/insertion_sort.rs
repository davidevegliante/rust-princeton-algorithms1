fn insertion_sort<T: PartialOrd>(data: &mut [T]){

    for i in 0..data.len(){
        let mut j = i;
        while j > 0 && data[j] < data[j-1] {
            data.swap(j, j-1);
            j = j - 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn already_sorted() {
        let mut data = [1, 2, 3, 4];
        insertion_sort(&mut data);
        assert_eq!(&[1, 2, 3, 4], &data);
    }

    #[test]
    fn simple_slice() {
        let mut data = [4, 3, 2, 1];
        insertion_sort(&mut data);
        assert_eq!(&[1, 2, 3, 4], &data);
    }

    #[test]
    fn char_slice() {
        let mut data = ["a", "c", "b", "f"];
        insertion_sort(&mut data);
        assert_eq!(&["a", "b", "c", "f"], &data);
    }
}