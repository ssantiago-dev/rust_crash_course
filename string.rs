// Simple Rust program
fn main() 
{
    let str: &str = "Welcome to Rust";
    let mut string: String = String::from("Welcome to Rust");
    let slice = &string[..6];

    println!("{}", slice.len());

    string.push('-');
    string.push_str("Ruim para Blockchain");
    string = string.replace("Rust", "Go");

    println!("{}", string);
}