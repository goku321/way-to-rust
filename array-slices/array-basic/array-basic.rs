// use std::mem;

fn main() {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];

    println!("First element of array is: {}", xs[0]);

    let ys: [i32; 500] = [0; 500];

    println!("First element of ys is: {}", ys[0]);

    // use `len` to return size of the array
    println!("array size: {}", ys.len())
}