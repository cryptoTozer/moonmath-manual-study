
/* 
Find an 𝑚∈\Z and an 𝑟∈\N with 0≤𝑟<|𝑏| such that  𝑎=𝑚⋅𝑏+𝑟

Terminology:
a -> dividend
b -> divisor
m -> quotient
r -> remainder

*/
pub fn long_division(dividend: i64, divisor: i64) -> (i64, i64) {
    let dividend_abs = dividend.abs() as usize;
    let divisor_abs = divisor.abs() as usize;

    let dividend_string = dividend_abs.to_string();
    let mut digits = Vec::new();
    for c in dividend_string.chars() {
        let digit = c.to_digit(10).unwrap();
        digits.push(digit);
    }
    println!("DIGITS: {:?}", digits);

    let mut quotient = 0;
    let mut num = 0;

    for i in 0..digits.len() {
        num = num * 10 + digits[i] as usize;

        if num >= divisor_abs {
            let mut count = 0;
            while num >= divisor_abs * (count + 1) {
                count += 1;
            }

            quotient = quotient * 10 + count;
            num -= divisor_abs * count;
        } else if quotient > 0 {
            quotient *= 10;
        }
    }

    let remainder = num as i64;

    let sign = (dividend < 0) ^ (divisor < 0);
    let quotient_signed = (quotient as i64) * if sign { -1 } else { 1 };
    let quotient_adjusted = if dividend < 0 {
        quotient_signed - 1
    } else {
        quotient_signed
    };
    let remainder_adjusted = if dividend < 0 {
        divisor.abs() - remainder
    } else {
        remainder
    };

    (quotient_adjusted, remainder_adjusted)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let (quotient, remainder) = long_division(27, 5);
        assert_eq!((quotient, remainder), (5, 2));

        let (quotient, remainder) = long_division(27, -5);
        assert_eq!((quotient, remainder), (-5, 2));

        let (quotient, remainder) = long_division(-27, 5);
        assert_eq!((quotient, remainder), (-6, 3));

        let (quotient, remainder) = long_division(-1687, 11);
        assert_eq!((quotient, remainder), (-154, 7));
    }
}