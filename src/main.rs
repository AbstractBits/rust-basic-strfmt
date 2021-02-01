fn main() {

    // basic
    let greeting_format = format!("{} {}!", "Hello", "World");
    println!("{}", greeting_format); // Hello World!

	// positional parameters
    let positional_format = format!("Languages: [{1}, {0}]", "Go", "Rust");
    println!("{}", positional_format); // Languages: [Rust, Go]

    // named parameters
    let named_format = format!("{x} + {y} = {result}", x=8, y=6, result=8 + 6);
    println!("{}", named_format); // 8 + 6 = 14
}
