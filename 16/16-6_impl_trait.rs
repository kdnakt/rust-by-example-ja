use std::iter;
use std::vec::IntoIter;

// ===================
// As an argument type
// ===================

// This function takes any type R which implements BufRead,
// but it actually doesn't care about the concrete type R.
fn parse_csv_document_traditional<R: std::io::BufRead>(src: R) -> std::io::Result<Vec<Vec<String>>> {
    src.lines()
        .map(|line| {
            // For each line in the source
            line.map(|line| {
                // If the line was read successfully, process it, if not, return the error
                line.split(',') // Split the line separated by commas
                    .map(|entry| String::from(entry.trim())) // Remove leading and trailing whitespace
                    .collect() // Collect all strings in a row into a Vec<String>
            })
        })
        .collect() // Collect all lines into a Vec<Vec<String>>
}

fn parse_csv_document(src: impl std::io::BufRead) -> std::io::Result<Vec<Vec<String>>> {
    parse_csv_document_traditional(src)
}

// ================
// As a return type
// ================

// This function combines two `Vec<i32>` and returns an iterator over it.
// Look how complicated its return type is!
fn combine_vecs_explicit_return_type(
    v: Vec<i32>,
    u: Vec<i32>,
) -> iter::Cycle<iter::Chain<IntoIter<i32>, IntoIter<i32>>> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// This is the exact same function, but its return type uses `impl Trait`.
// Look how much simpler it is!
fn combine_vecs(
    v: Vec<i32>,
    u: Vec<i32>,
) -> impl Iterator<Item=i32> {
    v.into_iter().chain(u.into_iter()).cycle()
}

// Returns a function that adds `y` to its input
fn make_adder_function(y: i32) -> impl Fn(i32) -> i32 {
    let closure = move |x: i32| { x + y };
    closure
}

// A traditional edition of make_adder_function.
fn make_adder_function_traditional(y: i32) -> Box<dyn Fn(i32) -> i32> {
    let closure = move |x: i32| { x + y };
    Box::new(closure)
}

fn double_positives<'a>(numbers: &'a Vec<i32>) -> impl Iterator<Item=i32> + 'a {
    numbers
        .iter()
        .filter(|x| x > &&0)
        .map(|x| x * 2)
}

fn larger_than_zero(n: &&i32) -> bool {
    n > &&0
}

fn times_two(n: &i32) -> i32 {
    n * &2
}

// A traditional edition of double_positives.
// A closure has an unnamed type of its own, so higher-order functions such as filter and map are
// not easy to handle without `impl Traits`.
// In the following example, a traditional edition of double_positives, closures are replaced with
// normal functions to write out an explicit return type.
fn double_positives_traditional<'a>(
    numbers: &'a Vec<i32>
) -> iter::Map<iter::Filter<std::slice::Iter<'a, i32>, fn(&&i32) -> bool>, fn(&i32) -> i32> {
    numbers
        .iter()
        .filter(larger_than_zero as fn(&&i32) -> bool)  // convert larger_than_zero to a function pointer
        .map(times_two)
}

fn main() {
    let csv = "\
        Id,Language,Greeting
        0,Japanese,こんにちは
        1,English,Hello
        2,French,Bonjour
        3,German,Guten Tag";
    // parse_csv_document_traditional parses a csv document.
    if let Ok(result) = parse_csv_document_traditional(csv.as_bytes()) {
        println!("{:?}", result);
    }
    // parse_csv_document_traditional with our beloved turbofish.
    if let Ok(result) = parse_csv_document_traditional::<&[u8]>(csv.as_bytes()) {
        println!("{:?}", result);
    }
    // parse_csv_document behaves the same.
    if let Ok(result) = parse_csv_document(csv.as_bytes()) {
        println!("{:?}", result);
    }
    // A turbofish cannot follow parse_csv_document because the function is written with `impl Trait`.
    // let result = parse_csv_document::<&[u8]>(csv.as_bytes());

    // combine_vecs_explicit_return_type consumes two vectors and returns a new iterator over them.
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    print!("combine_vecs_explicit_return_type: ");
    combine_vecs_explicit_return_type(v1, v2).take(10).for_each(|e| {
        print!("{}, ", e);
    });
    println!("");

    // combine_vecs behaves the same as combine_vecs_explicit_return_type.
    let v1 = vec![1, 2, 3];
    let v2 = vec![4, 5];
    print!("combine_vecs: ");
    combine_vecs(v1, v2).take(10).for_each(|e| {
        print!("{}, ", e);
    });
    println!("");

    // make_adder_function takes a number and returns a new function.
    let plus_one = make_adder_function(1);
    println!("make_adder_function(1)(2): {}", plus_one(2));    // prints 3
    // make_adder_function_traditional behaves the same as make_adder_function.
    let plus_one = make_adder_function_traditional(1);
    println!("make_adder_function_traditional(1)(2): {}", plus_one(2));    // prints 3

    // double_positives takes a vector and returns a new vector.
    let singles = vec![-3, -2, 2, 3];
    let doubles = double_positives(&singles);
    println!("double_positives([-3, -2, 2, 3]): {:?}", doubles.collect::<Vec<i32>>());    // prints [4, 6]
    // double_positives_traditional behaves the same as double_positives.
    let doubles = double_positives_traditional(&singles);
    println!("double_positives_traditional([-3, -2, 2, 3]): {:?}", doubles.collect::<Vec<i32>>());    // prints [4, 6]
}
