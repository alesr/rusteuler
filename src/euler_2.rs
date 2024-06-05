use std::f64;

const INPUT_EULER_2: u64 = 4_000_000;

// Euler 2: Even Fibonacci numbers.

pub fn run() {
    println!("Euler 2: {}", even_fibonacci(INPUT_EULER_2));
}

// Constants for Binet's formula.
lazy_static! {
    static ref SQRT_5: f64 = 5.0f64.sqrt();
    static ref PHI: f64 = (1.0 + *SQRT_5) / 2.0;
    static ref PSI: f64 = (1.0 - *SQRT_5) / 2.0;
}

// Returns the nth Fibonacci number using Binet's formula.
fn fibonacci(n: i32) -> u64 {
    ((PHI.powi(n) - PSI.powi(n)) / *SQRT_5).round() as u64
}

// Calculates the sum of even Fibonacci numbers up to a given limit.
fn even_fibonacci(limit: u64) -> u64 {
    let mut sum = 0;
    let mut i = 3;

    loop {
        let fib = fibonacci(i);
        if fib > limit {
            break;
        }

        sum += fib;
        i += 3; // Only even indices of Fibonacci sequence.
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solution() {
        let result = even_fibonacci(INPUT_EULER_2);
        assert_eq!(result, 4613732);
    }
}
