fn fibonacci(n: i32) -> i32 {
    // base case
    if n == 0 || n == 1 {
        n
    } else {
        fibonacci(n-1) + fibonacci(n-2)
    }
}

fn main() {
    let mut sum = 0;
    let mut n = 0;
    loop {
        let f = fibonacci(n);
        if f >= 4_000_000 {
            break
        } else if f % 2 == 0 {
            sum += f;
        }
        n += 1;
    }

    println!("Sum of even fibs up to 4million: {sum}")
}