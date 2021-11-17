type ParseIntResult<T> = Result<T, std::num::ParseIntError>;

fn sum_strs(m: &str, n: &str) -> ParseIntResult<i32> {
    let parsed: i32 = m.parse()?;
    let turbo_parsed = n.parse::<i32>()?;
    Ok(parsed + turbo_parsed)
}

// Same to above but without the question mark operator
fn sum_strs2(m: &str, n: &str) -> ParseIntResult<i32> {
    let result_parsed: ParseIntResult<i32> = m.parse();
    match result_parsed {
        Result::Ok(parsed) => {
            let result_turbo_parsed = n.parse::<i32>();
            match result_turbo_parsed {
                Result::Ok(turbo_parsed) => Ok(parsed + turbo_parsed),
                Result::Err(e) => Err(e),
            }
        },
        Result::Err(e) => Err(e),
    }
}

fn main() {
    let parsed: i32 = "5".parse().unwrap();
    let turbo_parsed = "10".parse::<i32>().unwrap();
    let sum = parsed + turbo_parsed;
    println!("Sum: {:?}", sum);
    // "a".parse::<i32>().unwrap(); // unwrap causes panic!

    let sum = sum_strs("5", "10");
    println!("Sum: {:?}", sum);
    let sum = sum_strs("a", "10");  // Returns error
    println!("Sum: {:?}", sum);

    let sum = sum_strs2("5", "10");
    println!("Sum: {:?}", sum);
    let sum = sum_strs2("a", "10");  // Returns error
    println!("Sum: {:?}", sum);
}
