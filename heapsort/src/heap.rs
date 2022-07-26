
pub struct Heap<T: Ord + Clone>{
    data: Vec<T>
}

impl<T: Ord + Clone> Heap<T>{
    pub fn new() -> Self {
        Heap{
            data: Vec::<T>::new(),
        }
    }

    pub fn from_vec(mut data: Vec<T>) -> Self {
        heap_construction(&mut data);
        let new_heap = Heap{ data: data };

        new_heap
    }

    pub fn add(&mut self, elem: T){

        self.data.push(elem);
        let new_size = self.data.len();
        swim(&mut self.data, new_size);
    }

    pub fn del_max(&mut self) -> Option<T>{

        match self.data.is_empty(){
            true => None,
            false => {
                let n = self.data.len();
                swap(&mut self.data, 1, n);
                let max = self.data.remove(self.data.len()-1);
                sink(&mut self.data, 1, n-1);
                Some(max)
            }
        }

    }

    pub fn max(&self) -> Option<&T>{

        match self.data.is_empty(){
            true => None,
            false => Some(&self.data[0])
        }
    }

    pub fn is_empty(&self) -> bool{

        self.data.is_empty()
    }

    pub fn size(&self) -> usize{

        self.data.len()
    }

    pub fn sort(data: &mut [T]){

        heap_construction(data);
        let mut n = data.len();
        while n > 1{
            swap(data, 1, n);
            n-=1;
            sink(data, 1, n);
        }
    }


}

impl<T: Ord + Clone> Iterator for Heap<T>{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.del_max()
    }
}

fn sink<T:Ord>(data: &mut [T], elem: usize, bound: usize){

    let n = bound;
    let mut i = elem;

    // if we consider indices starting from 1, we have to iterate half vec
    while 2*i <= n {

        // select the greater child
        let mut j = 2 * i;
        if j < n && less(data, j, j+1){
            j+=1;
        }

        // if parent is bigger than bigger child, we can stop
        if less(data, j, i){
            break;
        }

        swap(data, i, j);
        i = j;
    }
}

fn heap_construction<T:Ord>(data: &mut [T]){

    let n = data.len();
    let mut i = n / 2;

    while i >= 1{
        sink(data, i, n);
        i-=1;
    }
}

fn swim<T:Ord>(data: &mut [T], elem: usize){
    let mut i = elem;

    while i > 1 && less(data, i/2, i){
        swap(data, i/2, i);
        i/=2;
    }
}

fn less<T:Ord>(data: &mut [T], i: usize, j: usize) -> bool{
    // utility function. Fix the indices of the comparison
    data[i-1] < data[j-1]
}

fn swap<T>(data: &mut [T], i: usize, j: usize){
    // utility function. Fix the indices of the swap
    data.swap(i-1, j-1);
}

mod test {
    use super::*;

    #[test]
    fn heap_creation() {
        let heap = Heap::<i32>::new();

        let mut things: Vec<i32> = vec![0, 1, 2];
        let mut heap = Heap::from_vec(things);

        assert_eq!(heap.del_max(), Some(2));
        assert_eq!(heap.del_max(), Some(1));
        assert_eq!(heap.del_max(), Some(0));
        assert_eq!(heap.del_max(), None);
    }


    #[test]
    fn easy_build_heap_test() {
        use super::*;

        let mut things = vec![4, 1, 3, 2, 16, 9, 10, 14, 8, 7];
        let mut heap = Heap::from_vec(things);
        //assert_eq!(things, [16, 14, 10, 8, 7, 9, 3, 2, 4, 1]);
        assert_eq!(heap.del_max(), Some(16));
        assert_eq!(heap.del_max(), Some(14));
        assert_eq!(heap.del_max(), Some(10));
        assert_eq!(heap.del_max(), Some(9));
        assert_eq!(heap.del_max(), Some(8));
        assert_eq!(heap.del_max(), Some(7));
        assert_eq!(heap.del_max(), Some(4));
        assert_eq!(heap.del_max(), Some(3));
        assert_eq!(heap.del_max(), Some(2));
        assert_eq!(heap.del_max(), Some(1));
        assert_eq!(heap.del_max(), None);
    }


    #[test]
    fn create_empty_heap() {
        let mut heap = Heap::<i32>::new();
        assert_eq!(heap.is_empty(), true);
        assert_eq!(heap.max(), None);
        assert_eq!(heap.size(), 0);
        assert_eq!(heap.del_max(), None);
    }

    #[test]
    fn max_test() {
        let things = vec![4, 2, 3, 16, 9, 10, 14, 1, 7];
        let mut heap = Heap::from_vec(things);
        assert_eq!(heap.max(), Some(&16));
        assert_eq!(heap.del_max(), Some(16));
    }

    #[test]
    fn test_insert() {
        let mut heap = Heap::new();
        heap.add(5);
        heap.add(6);
        heap.add(7);
        heap.add(3);
        heap.add(10);
        heap.add(9);
        assert_eq!(heap.del_max(), Some(10));
        assert_eq!(heap.del_max(), Some(9));
        assert_eq!(heap.del_max(), Some(7));
        assert_eq!(heap.del_max(), Some(6));
        assert_eq!(heap.del_max(), Some(5));
    }

    #[test]
    fn test_sort() {
        let mut things = [1, 5, 0, 2, 4, 3, 6, 8, 8, 1];
        Heap::sort(&mut things);
        assert_eq!(things, [0, 1, 1, 2, 3, 4, 5, 6, 8, 8])
    }
}