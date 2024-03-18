fn main() {
    println!("Hello World");
    print!("Hello");
}

// This line uses the println! macro. It takes a string literal ("Hello World" in this case) and prints it to the console.
// Crucially, println! also adds a newline character (\n) at the end. This means the output will be "Hello World" followed by a jump to the next line.
// print!("Hello");

// This line uses the print! macro, similar to println!. It also takes a string literal ("Hello" here).
// However, print! only prints the provided string without adding any newline character.
