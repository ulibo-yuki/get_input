use std::io::{stdout, Write};

/// get user's input. return String.
/// 
/// ## how to get input without message.
/// ```
/// let input = get_input("");
/// ```
/// 
/// ## how to get input with message.
/// ```
/// let input = get_input("please title>");
/// ```
/// 
/// ```bash
/// // 👇console
/// please title>foo 👈foo is user's input.
/// // input == "foo"
/// ```
pub fn get_input(message: &str) -> String {
    print!("{}>", message);
    stdout().flush().unwrap();
    let mut word = String::new();
    std::io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");
        word.trim().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(get_input("hello?>"), "hoge".to_string());
    }
}
