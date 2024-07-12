// If we list all the natural numbers below $10$ that are multiples of $3$ or $5$,
// we get $3, 5, 6$ and $9$. The sum of these multiples is $23$.
// Find the sum of all the multiples of $3$ or $5$ below $1000$


// pseudocode
// 1. loop through the range of numbers given
// 2. for every element/number determine if it's a multiple of 3 and 5
// 3. get the sum of all the multiples

pub fn multiples_of_three_and_five() -> i32 {
    let mut sum: i32 = 0;
    for i in 0..1000 {
        if i % 3 == 0 || i % 5 == 0 {
            sum +=i;
        }
    }
    println!("{sum}");
    return sum;
}