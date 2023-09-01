/// Returns a string with "Hello"
///
/// # Returns
/// `String` with "Hello"
///
/// ## Examples
/// ```
/// use helloer::return_hello;
/// assert_eq!(return_hello(), String::from("Hello"))
/// ```
pub fn return_hello() -> String {
    println!("Lol");
    "Hello".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_return_hello() {
        assert_eq!(return_hello(), "Hello");
    }
}
