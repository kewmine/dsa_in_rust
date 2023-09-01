use std::time::{Instant};

pub fn insertion_sort(mut vec:Vec<i32>) {
    let start = Instant::now();
    for  i in 1..vec.len() {
        let key = vec[i];
        let mut j = i;

        while j>0 && key < vec[j-1] {
            vec[j] = vec[j - 1];
            j -= 1;
        }
        vec[j] = key;
    }
    let duration  = start.elapsed().as_nanos();

    println!("sorted_vector:\t\t{:?}", &vec);
    println!("duration: {:?}ns\n", duration);
}