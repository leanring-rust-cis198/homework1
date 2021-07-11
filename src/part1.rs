/*
    CIS198 Homework 1
    Part 1: Implementing functions

    Complete and write at least one unit test for each function you implement.
    If it already has a unit test, either add assertions to it or add a new one.
    Also answer the questions in text.
*/

// Remove these once you are done editing the file!
// This will result in useful warnings if you missed something.

/*
    Problem 1: Double

    Implement the function that doubles an integer in three different ways.

    What are some differences between them? Can you write unit tests
    which fail (or fail to compile) for some but not others?

    Which of the three do you prefer?
*/

pub fn double_v1(n: i32) -> i32 {
    return n*2;
}

pub fn double_v2(n: &i32) -> i32 {
    return n*2;
}

pub fn double_v3(n: &mut i32) {
    // double n in place
    *n = *n * 2;
}

// Example unit test (so you can recall the syntax)
#[test]
fn test_double_v1() {
    assert_eq!(double_v1(2), 4);
    assert_eq!(double_v1(-3), -6);
}
#[test]
fn test_double_v2() {
    assert_eq!(double_v2(&2), 4);
    assert_eq!(double_v2(&-3), -6);
}
#[test]
fn test_double_v3() {
    let mut a:i32 = 2;
    let mut b:i32 = -3;
    double_v3(&mut a);
    double_v3(&mut b);
    assert_eq!(a, 4);
    assert_eq!(b, -6);
}

/*
    Problem 2: Integer square root

    Implement the integer square root function: sqrt(n) should return the
    largest m such that m * m <= n. For a 'harder' version, try to do it more
    efficiently than trying every possibility.
*/
pub fn sqrt(n: usize) -> usize {
    for m in 0..n {
        if m*m == n {
            return m;
        }
    }
    panic!("no sqaure root found");
}

// Remember to write unit tests here (and on all future functions)
#[test]
fn test_sqrt() {
    assert_eq!(sqrt(4), 2);
}

/*
    Problem 3: Slice sum

    Implement the sum function on slices in two different ways
    (using different for loop patterns).
    Do not use the predefined sum function.
    Also, try to do it without an unnecessary `return` statement at the end --
    Clippy should detect if you mess this up.

    Which of the two ways do you prefer?
*/
pub fn sum_v1(slice: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for &v in slice {
        sum += v;
    }
    return sum;
}

pub fn sum_v2(slice: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for v in slice {
        sum += v;
    }
    return sum;
}

/*
    Problem 4: Unique

    Make unique. Create a new vector which contains each item in the vector
    only once! Much like a set would.
    This doesn't need to be efficient; you can use a for loop.
*/

pub fn unique(slice: &[i32]) -> Vec<i32> {
    let mut unique_vec: Vec<i32> = vec![];
    for v in slice.iter() {
        if !unique_vec.contains(v) {
            unique_vec.push(*v);
        }
    }
    return unique_vec;
}

#[test]
fn test_unique() {
    assert_eq!(unique(&[1, 2, 3, 3, 2]), vec![1, 2, 3]);
}

/*
    Problem 5: Filter

    Return a new vector containing only elements that satisfy `pred`.
    This uses some unfamiliar syntax for the type of pred -- all you need
    to know is that pred is a function from i32 to bool.
*/
pub fn filter(slice: &[i32], pred: impl Fn(i32) -> bool) -> Vec<i32> {
    let mut unique_vec: Vec<i32> = vec![];
    for &v in slice.iter() {
        if pred(v) {
            unique_vec.push(v);
        }
    }
    return unique_vec;
}

#[test]
fn test_filter() {
    fn is_even(n: i32) -> bool {
        n % 2 == 0
    }
    assert_eq!(filter(&vec![1, 2, 3, 4, 5, 6], &is_even), vec![2, 4, 6]);
}

/*
    Problem 6: Fibonacci

    Given starting fibonacci numbers n1 and n2, compute a vector of
    length 'out_size'
    where v[i] is the ith fibonacci number.
*/
pub fn fibonacci(n1: i32, n2: i32, out_size: usize) -> Vec<i32> {
    if out_size == 0 {
        return vec![];
    }

    if out_size == 1 {
        return vec![n1];
    }

    if out_size == 2 {
        return vec![n1, n2];
    }

    return [n1].iter().chain(
            &fibonacci(n2, n1+n2, out_size - 1)
        ).map(|&x|x).collect();
}

#[test]
fn fibonacci_test() {
    assert_eq!(fibonacci(1, 1, 0), []);
    assert_eq!(fibonacci(0, 0, 5), [0, 0, 0, 0, 0]);
    assert_eq!(fibonacci(1, 1, 10), [1, 1, 2, 3, 5, 8, 13, 21, 34, 55]);
}

/*
    Problem 7: String concatenation

    Create a function which concats 2 &strs and returns a String,
    and a function which concats 2 Strings and returns a String.

    You may use any standard library function you wish.

    What are some reasons the second function is not efficient?
*/
pub fn str_concat(s1: &str, s2: &str) -> String {
    return s1.to_owned() + s2;
}

pub fn string_concat(s1: String, s2: String) -> String {
    return s1.to_owned() + &s2;
}

#[test]
fn str_concat_test() {
    assert_eq!(str_concat("1", "2"), "12");
    assert_eq!(str_concat("abcd", "efgh"), "abcdefgh");
}

#[test]
fn string_concat_test() {
    assert_eq!(string_concat("1".to_owned(), "2".to_owned()), "12");
    assert_eq!(string_concat("abcd".to_owned(), "efgh".to_owned()), "abcdefgh");
}

/*
    Problem 8: String concatenation continued

    Convert a Vec<String> into a String.
    Your answer to the previous part may help.
*/

pub fn concat_all(v: Vec<String>) -> String {
    let mut ret = "".to_owned();
    for s in v {
        ret += &s;
    }
    return ret;
}

#[test]
fn concat_all_test() {
    assert_eq!(concat_all(
        vec!["1".to_owned(), "23".to_owned(), "456".to_owned()]
    ), "123456");
}

/*
    Problem 9: Parsing

    Convert a Vec<String> into a Vec<i32> and vice versa.

    Assume all strings are correct numbers! We will do error handling later.
    Use `.expect("ignoring error")` to ignore Result from parse()
    See https://doc.rust-lang.org/std/primitive.str.html#method.parse

    The unit tests check if your functions are inverses of each other.

    A useful macro: format! is like println! but returns a String.
*/

pub fn parse_all(v: Vec<String>) -> Vec<i32> {
    let mut ret: Vec<i32> = vec![];
    for s in v {
        ret.push(s.parse().unwrap());
    }
    return ret;
}

pub fn print_all(v: Vec<i32>) -> Vec<String> {
    let mut ret: Vec<String> = vec![];
    for s in v {
        ret.push(s.to_string());
    }
    return ret;
}

#[test]
fn test_print_parse() {
    assert_eq!(parse_all(print_all(vec![1, 2])), vec![1, 2]);
}

#[test]
fn test_parse_print() {
    let v = vec!["1".to_string(), "2".to_string()];
    assert_eq!(print_all(parse_all(v.clone())), v);
}

/*
    Problem 10: Composing functions

    Implement a function which concatenates the even Fibonacci
    numbers out of the first n Fibonacci numbers.

    For example: if n = 6, the first 5 Fibonacci numbers are 1, 1, 2, 3, 5, 8,
    so the function should return the String "28".

    Don't use a for loop! Your previous functions should be sufficient.
*/

pub fn concat_even_fibonaccis(n: usize) -> String {
    let fib = fibonacci(1, 1, n);
    let fib_even = filter(&fib, |x|x%2==0);
    let fib_even_string = print_all(fib_even);
    return concat_all(fib_even_string);
}

#[test]
fn test_concat_even_fibonaccis() {
    assert_eq!(&concat_even_fibonaccis(6), "28");
    assert_eq!(&concat_even_fibonaccis(9), "2834");
}
