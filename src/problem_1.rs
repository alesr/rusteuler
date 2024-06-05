const INPUT: i32 = 1000;

// Problem 1: Multiples of 3 and 5.

pub fn run() {
    println!("Problem 1: {}", multiples_of_3_5(INPUT));
}

fn multiples_of_3_5(count: i32) -> i32 {
    let mut sum = 0;

    for i in 0..count {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(multiples_of_3_5(10), 23);
    }

    #[test]
    fn solution() {
        assert_eq!(multiples_of_3_5(INPUT), 233168);
    }
}
