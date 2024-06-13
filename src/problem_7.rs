const INPUT: i32 = 10_001;

// Problem 7: 10001st Prime

pub fn run() {
    println!("Problem 7: {}", nth_prime(INPUT));
}

fn nth_prime(n: i32) -> i32 {
    let mut count = 0;
    let mut i = 3;

    while count < n - 1 {
        if is_prime(i) {
            count += 1;
        }
        i += 2;
    }
    i - 2
}

fn is_prime(n: i32) -> bool {
    let sqrt_n = (n as f64).sqrt() as i32;

    for i in 2..=sqrt_n {
        if n % i == 0 {
            return false;
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(nth_prime(6), 13);
    }

    #[test]
    fn solution() {
        assert_eq!(nth_prime(INPUT), 104743);
    }
}
