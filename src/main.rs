mod algorithms;
mod utils;

use crate::utils::gen_vec;

fn main() {
    let unsorted_vector = gen_vec(10);
    algorithms::insertion_sort::insertion_sort(unsorted_vector);
}
