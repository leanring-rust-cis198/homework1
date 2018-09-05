// Part 2.
// Make the following failing functions/tests pass.
// Answer the questions as a comment next to the problems.

// Problem 1.
// Create functions split_ref and split_clone such that
// all the following tests will pass. Feel free to use Rust's split method
// (https://doc.rust-lang.org/std/primitive.slice.html#method.split)
// as needed.
#[test]
fn split_ref_tests(){
    let string = "Hello World!".to_string();
    assert_eq!(split_ref(& string), ["Hello", "World!"]);
    assert_eq!(split_ref("Hello World!"), & ["Hello", "World!"]);
    assert_eq!(split_ref("Hello World!"), vec!["Hello", "World!"]);
}

#[test]
fn split_clone_tests(){
    let string = "Hello World!".to_string();
    assert_eq!(split_clone(& string), ["Hello", "World!"]);
    assert_eq!(split_clone("Hello World!"), & ["Hello", "World!"]);
    assert_eq!(split_clone("Hello World!"), vec!["Hello", "World!"]);
}

// Problem 2.
// Write function pick_longest which picks the longests of two string-ish
// objects. Please make the function as general as possible (i.e do not
// take "String" as a parameter).
//
// From simplicity return a new String, later we will learn how to return
// references. Write additional tests.
//

#[test]
fn pick_longest_tests() {
    assert_eq!(pick_longest("cat".to_string(), "dog".to_string()), "cat");
}

// Extra
//For the curious, attempt to return reference, that is:
//
// pick_longerst(???) -> &str
//
// What goes wrong when you try to implement this function? Why is this
// the case?
// End Extra


// Problem 3.
// Write a function that returns all the contents of a file as a single String.
// Do not use std::fs::read_to_string
// Use the File::open, and the read_to_string
// (https://doc.rust-lang.org/std/io/trait.Read.html#method.read_to_string)
// functions to implement.
// Use .expect("ignoring error: ") to ignore the Result<...> type in open() and
// read_to_string. We will handle error handling later.
fn print_contents_of_file_original(path : &str) -> String {
    unimplemented!()
}

// Problem 4.
// Why does the following implementation not work as expected?
// Fix by changing the type signature of add1 and the way it's called on add1_test().
// do NOT change the return type.

#[test]
fn add1_test() {
    let mut x = 1;
    add1(x);
    assert_eq!(1, 2);
}

fn add1(mut x : i32) -> () {
    x += 1;
}

// Problem 5.
// Error says: cannot assign to immutable borrowed content `*str1`
// But we declared it mutable? Fix by changing only the line below.
fn mut2() {
    let hello = String::from("hello");

    // CHANGE ONLY THIS LINE:
    let mut str1: & String = & String::from("str1");

    *str1 = hello;
}