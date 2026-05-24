fn greetings(name: &str) -> String {
    return format!("Hello {}", name);
}

fn main() {
    println!("{}", greetings("world"))
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_greeting_world() {
        assert_eq!(greetings("world"), "Hello world");
    }

    #[test]
    fn test_greeting_custom() {
        assert_eq!(greetings("Tokyo"), "Hello Tokyo");
    }
}
