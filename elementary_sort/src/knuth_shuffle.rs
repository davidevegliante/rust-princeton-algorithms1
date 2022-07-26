use rand::Rng;

pub fn knuth_shuffling<T>(data: &mut [T]){

    for i in 0..data.len(){
        let rand_i = rand::thread_rng().gen_range(0..i+1);
        data.swap(i, rand_i);
    }

}
