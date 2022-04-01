pub fn function() {
    println!("called `my2::nested::function()`");
}

#[allow(dead_code)]
fn private_function() {
    println!("called `my2::nested::private_function()`");
}
