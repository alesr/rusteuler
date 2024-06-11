const NUMBER_OF_DIGITS: u32 = 3;

// Problem 4: Largest Palindrome Product

pub fn run() {
    println!(
        "Problem 4: {}",
        largest_palindrome_product(NUMBER_OF_DIGITS)
    );
}

fn largest_palindrome_product(number_of_digits: u32) -> u32 {
    let factor_max = 10_u32.pow(number_of_digits) - 1;
    let factor_min = 10_u32.pow(number_of_digits - 1);

    let mut result: u32 = 0;

    for factor_a in (factor_min..=factor_max).rev() {
        for factor_b in (factor_min..=factor_a).rev() {
            let prod = factor_a * factor_b;
            let prod_str = prod.to_string();
            let prod_str_rev = reverse_string(&prod_str);

            if prod_str == prod_str_rev && prod > result {
                result = prod;
            }
        }
    }

    result
}

fn reverse_string(s: &str) -> String {
    s.chars().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reverse_string_test() {
        assert_eq!(reverse_string("1234"), "4321");
    }

    #[test]
    fn example() {
        assert_eq!(largest_palindrome_product(2), 9009);
    }

    #[test]
    fn solution() {
        assert_eq!(largest_palindrome_product(NUMBER_OF_DIGITS), 906609);
    }
}
