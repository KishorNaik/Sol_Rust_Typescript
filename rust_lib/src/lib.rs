use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    let result:i32 = a + b;
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_positive_numbers() {
        assert_eq!(add(3, 5), 8);
    }

    #[test]
    fn test_add_negative_numbers() {
        assert_eq!(add(-3, -5), -8);
    }

    #[test]
    fn test_add_mixed_numbers() {
        assert_eq!(add(-3, 5), 2);
    }

    #[test]
    fn test_add_zero() {
        assert_eq!(add(0, 0), 0);
    }

    #[test]
    fn test_add_large_numbers() {
        assert_eq!(add(1_000_000, 2_000_000), 3_000_000);
    }
}