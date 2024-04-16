use rand::Rng;

// misc
pub fn gen_vec(size:i32) -> Vec<i32> {

    let mut rng = rand::thread_rng();
    let mut vector = vec![];
    for _i in 0..size {
        vector.push(rng.gen_range(0..20));
    }
    vector
}