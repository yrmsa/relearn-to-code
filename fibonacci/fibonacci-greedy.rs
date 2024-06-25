fn main() {
    println!("{}", fibonacci(0));
}

// this is how i code with c
// =========================
// vvvvvvvvvvvvvvvvvvvvvvvvv
// fn fibonacci(n: u128) -> u128 {
//     if n < 3 {
//         return 1;
//     }

//     return fibonacci(n - 1) + fibonacci(n - 2);
// }

// the rust way - with fancy pattern matching
fn fibonacci(n: usize) -> usize {
    match n {
        0 => panic!("errorr... fibonacci starts at 1"),
        1 | 2 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
