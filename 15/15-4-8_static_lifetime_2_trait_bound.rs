use std::fmt::Debug;

static J: i32 = 42;

fn print_it( input: impl Debug + 'static ) {
    println!( "'static value passed in is: {:?}", input );
}

fn main() {
    // i is owned and contains no references, thus it's 'static:
    let i = 5;
    print_it(i);
    
    let static_int = &J;

    // oops, &i only has the lifetime defined by the scope of
    // main(), so it's not 'static:
    print_it(static_int);
}
