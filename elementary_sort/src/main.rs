use elementary_sort::knuth_shuffle::knuth_shuffling;

fn main(){
    let mut data = [1,2,3,4, 5, 6, 7, 8, 9, 10];
    println!("Original {:?}", data);
    knuth_shuffling(&mut data);
    println!("Shuffled {:?}", data);
}