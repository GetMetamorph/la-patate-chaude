pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub mod challenge;
pub mod message_struct;
pub mod hash_code;
pub mod recover_secret;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(2, 2), 4);
    }
}