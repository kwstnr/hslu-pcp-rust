fn main() {
    println!("Hello, world!");

    let f_ten = fibonacci(10);
    println!("Fibonacci of 10 is: {}", f_ten);

    let f_twenty = fibonacci(20);
    println!("Fibonacci of 20 is: {}", f_twenty);

    let mut memoized_fib = vec![0; 50];

    let f_memoized_thirty = fibonacci_memoized(30, &mut memoized_fib);
    println!("Fibonacci of 30 is: {}", f_memoized_thirty);

    let f_memoized_forty = fibonacci_memoized(40, &mut memoized_fib);
    println!("Fibonacci of 40 is: {}", f_memoized_forty);
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    fibonacci(n - 1) + fibonacci(n - 2)
}

fn fibonacci_memoized(n: u32, memo: &mut Vec<u32>) -> u32 {
    if n <= 1 {
        return n;
    }
    if memo[n as usize] != 0 {
        println!("had the {n}th value in the memo");
        return memo[n as usize];
    }

    println!("{n}th value not in the memo, calculating now");
    memo[n as usize] = fibonacci_memoized(n - 1, memo) + fibonacci_memoized(n - 2, memo);
    memo[n as usize]
}
