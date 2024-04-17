pub fn  binary_representation(mut number: usize) -> String {
    if number == 0 {
        return String::from("0");
    }

    let mut result = String::new();

    while number > 0 {
        let bit = (number & 1) as u8; // Get the least significant bit
        result.insert(0, if bit == 1 { '1' } else { '0' });
        number >>= 1; // Shift the number right by 1 to move to the next bit
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = binary_representation(7);
        assert_eq!(result, "111");

        let result = binary_representation(1);
        assert_eq!(result, "1");

        let result = binary_representation(8);
        assert_eq!(result, "1000");

        let result = binary_representation(15);
        assert_eq!(result, "1111");
    }
}