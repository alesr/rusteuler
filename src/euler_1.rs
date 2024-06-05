const INPUT_EULER_1: i32 = 1000;

// Euler 1: Multiples of 3 and 5.

pub fn run() {
    println!("Euler 1: {}", multiples_of_3_5(INPUT_EULER_1));
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
    fn solution() {
        let result = multiples_of_3_5(INPUT_EULER_1);
        assert_eq!(result, 233168);
    }
}
