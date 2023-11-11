// primitive_types3.rs
//
// Create an array with at least 100 elements in it where the ??? is.
//
// Execute `rustlings hint primitive_types3` or use the `hint` watch subcommand
// for a hint.

fn main() {
    // let a = vec![1; 100];
    // let a = [1; 100];

    // let mut a: Vec<i32> = Vec::new();
    // a.resize(100, 0);
    // a.fill(1);

    let mut a: Vec<i32> = Vec::with_capacity(100);
    for _ in 0..100 {
        a.push(1);
    }
    println!("{:?}", a.len());

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed")
    }
}
