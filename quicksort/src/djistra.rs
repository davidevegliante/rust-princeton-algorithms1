fn recursive_step<T: Ord>(data: &mut [T], lo: usize, hi: usize){
    if lo >= hi{
        return
    }

    let mut lt = lo;
    let mut gt = hi;
    let mut i = lo;

    while i <= gt {
        if data[i] < data[lo]{
            data.swap(i, lt);
            lt+=1;
            i+=1;
        }else if data[i] > data[lo]{
            data.swap(i, gt);
            gt-=1;
        }else{
            i+=1;
        }
    }
    recursive_step(data, lo, lt+1);
    recursive_step(data, gt+1, hi);
}