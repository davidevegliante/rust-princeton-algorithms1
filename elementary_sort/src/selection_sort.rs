fn selection_sort<T: PartialOrd>(data: &mut [T]) -> &[T] {
    
    for i in 0..data.len()-1{
        let min = min_index(&data[i..]);
        data.swap(i, i+min);
    }
    data
}

fn min_index<T: PartialOrd>(data: &[T]) -> usize {
    let mut min: usize = 0;
    for i in 0..data.len() {
        if data[i] < data[min] {
            min = i;
        }
    }
    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn already_sorted() {
        let mut data = [1, 2, 3, 4];
        selection_sort(&mut data);
        assert_eq!(&[1, 2, 3, 4], &data);
    }

    #[test]
    fn test_min() {
        let mut data = [5, 2, 4, 9, 1, 6];
        let min_i = min_index(&mut data);
        assert_eq!(min_i, 4);
    }

    #[test]
    fn test_min2() {
        let mut data = [4, 5, 2, 1, 1, 6, 9, 4, 2, 0];
        let min_i = min_index(&mut data);
        assert_eq!(min_i, 9);
    }

    #[test]
    fn simple_slice() {
        let mut data = [4, 3, 2, 1];
        selection_sort(&mut data);
        assert_eq!(&[1, 2, 3, 4], &data);
    }

    #[test]
    fn char_slice() {
        let mut data = ["a", "c", "b", "f"];
        selection_sort(&mut data);
        assert_eq!(&["a", "b", "c", "f"], &data);
    }
}
