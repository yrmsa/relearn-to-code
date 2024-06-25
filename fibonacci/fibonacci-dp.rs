fn main() {
    let number = 50;
    println!("{}", fibonacci(number, &mut vec![None; number + 1]));
}

// dynamic programming way - with memoization
fn fibonacci(n: usize, memo: &mut [Option<usize>]) -> usize {
    memo[n].unwrap_or_else(|| {
        let result = {
            match n {
                0 => panic!("errorr... fibonacci starts at 1"),
                1 | 2 => 1,
                _ => fibonacci(n - 1, memo) + fibonacci(n - 2, memo),
            }
        };
        memo[n] = Some(result);
        result
    })
}
