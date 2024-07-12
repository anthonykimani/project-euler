// 1. loop through a fibonacci sequence checking that the sequence value doesn't exceed four million
// 2. find sum of the even-valued terms

pub fn fibonacci_fn() -> i128 {
    let mut first_term = 1i128;
    let mut second_term = 2i128;
    let mut sum_of_even = 2i128;
    let mut sum= 0i128;
    while sum < 4_000_000 {
        sum = first_term + second_term;
        println!("{sum}");
        if sum % 2 == 0 {
            sum_of_even+=sum;
        }
        first_term = second_term;
        second_term = sum;
    }
    println!("sum of even fibonacci sequence: {sum_of_even}");
    return sum_of_even;
}