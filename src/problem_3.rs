const INPUT: u64 = 600851475143;

// Problem 3: Largest prime factor.

pub fn run() {
    println!("Problem 3: {}", largest_prime_factor(INPUT));
}

fn largest_prime_factor(mut n: u64) -> u64 {
    let mut result: u64 = 0;
    let mut i: u64 = 2;

    while i <= n {
        if n % i == 0 {
            result = i;
            n = n / i;
            i = 2;
        }

        i += 1;
    }

    result
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(largest_prime_factor(13195), 29);
    }

    #[test]
    fn solution() {
        assert_eq!(largest_prime_factor(INPUT), 6857);
    }
}
