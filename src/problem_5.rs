const INPUT: u32 = 20;

// Problem 5: Smallest Multiple

pub fn run() {
    println!("Problem 5: {}", smallest_multiple(INPUT));
}

fn smallest_multiple(n: u32) -> u32 {
    let mut smallest_multiple = 1;

    loop {
        let mut divisible = true;

        for i in 1..=n {
            if smallest_multiple % i != 0 {
                divisible = false;
                break;
            }
        }

        if divisible {
            return smallest_multiple;
        }

        smallest_multiple += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(smallest_multiple(10), 2520);
    }
    
    #[test]
    fn solution() {
        assert_eq!(smallest_multiple(INPUT), 232792560);
    }
}
