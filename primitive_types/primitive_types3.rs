// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

fn main() {
    let mut a: [u32; 150] = [0; 150];
    for i in 0..a.len() {
        a[i] = a[i] + (i as u32);
        println!("{}", a[i]);
    }

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}
