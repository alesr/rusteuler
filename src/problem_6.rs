const INPUT: u32 = 100;

// Problem 6: Sum Square Difference

pub fn run() {
    println!("Problem 6: {}", sum_square_difference(INPUT));
}

fn sum_square_difference(n :u32) -> u32 {
    let mut sum_of_sqrts: u32 = 0;
    let mut sum: u32 = 0;

    for i in 1..=n {
        sum_of_sqrts += i * i;
        sum += i;
    }
    (sum * sum) - sum_of_sqrts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(sum_square_difference(10), 2640);
    }

    #[test]
    fn solution() {
        assert_eq!(sum_square_difference(INPUT), 25164150);
    }
}
