fn main() {
    use helloer::return_hello;
    assert_eq!(return_hello(), String::from("Hello"));
    println!("{}", return_hello());
}
