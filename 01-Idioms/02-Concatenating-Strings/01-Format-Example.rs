fn say_hello(name: &str) -> String {
    // We could construct the result string manually.
    // let mut result = "Hello ".to_owned();
    // result.push_str(name);
    // result.push('!');
    // result

    // But using format! is better.
    format!("Hello {name}!")
}

/*
Disadvantages
It is usually not the most efficient way to combine strings - a series of
push operations on a mutable string is usually the most efficient
(especially if the string has been pre-allocated to the expected size).
 */