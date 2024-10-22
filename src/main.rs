pub fn add(x: i32, y: i32) -> i32 { x + y }

fn main() {
    println!("Hello, world! {}", add(2, 2));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 3), 5);
    }
}
