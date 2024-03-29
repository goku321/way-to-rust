use std::mem;

fn main() {
    let xs: [i32; 5] = [2, 2, 3, 4, 5];
    for item in xs.iter() {
        println!("item: {}", item);
    }

    let ys: [i32; 500] = [0; 500];

    println!("First element of ys is: {}", ys[0]);

    // use `len` to return size of the array
    println!("array size: {}", ys.len());

    // arrays are stack allocated
    println!("array occupies {} bytes", mem::size_of_val(&ys));
}