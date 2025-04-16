pub fn negative_spell(n: i64) -> String {
    // Check if the number is zero
    if n == 0 {
        return String::from("zero");
    }
    
    // Check if the number is positive
    if n > 0 {
        return String::from("error: positive number");
    }
    
    // Handle special case for -1,000,000
    if n == -1_000_000 {
        return String::from("minus one million");
    }
    
    // Get the absolute value of the number
    let abs_n = n.abs();
    
    // Convert to words and add "minus" prefix
    format!("minus {}", spell_number(abs_n))
}

// Helper function to spell out a positive number
fn spell_number(n: i64) -> String {
    if n == 0 {
        return String::from("zero");
    }
    
    let mut result = String::new();
    
    // Handle thousands (1,000 - 999,999)
    if n >= 1000 {
        let thousands = n / 1000;
        result.push_str(&spell_number(thousands));
        result.push_str(" thousand");
        
        if n % 1000 > 0 {
            result.push_str(" ");
        } else {
            return result; // Return early if there's nothing more to add
        }
    }
    
    // Handle hundreds (100-999)
    let remainder = n % 1000;
    if remainder >= 100 {
        let hundreds = remainder / 100;
        result.push_str(&spell_number(hundreds));
        result.push_str(" hundred");
        
        if remainder % 100 > 0 {
            result.push_str(" ");
        } else {
            return result; // Return early if there's nothing more to add
        }
    }
    
    // Handle tens and ones (0-99)
    let remainder = remainder % 100;
    match remainder {
        0 => {}, // Nothing to add
        1..=9 => {
            // Single digit numbers
            result.push_str(match remainder {
                1 => "one",
                2 => "two",
                3 => "three",
                4 => "four",
                5 => "five",
                6 => "six",
                7 => "seven", 
                8 => "eight",
                9 => "nine",
                _ => unreachable!(),
            });
        },
        10 => result.push_str("ten"),
        11 => result.push_str("eleven"),
        12 => result.push_str("twelve"),
        13 => result.push_str("thirteen"),
        14 => result.push_str("fourteen"),
        15 => result.push_str("fifteen"),
        16 => result.push_str("sixteen"),
        17 => result.push_str("seventeen"),
        18 => result.push_str("eighteen"),
        19 => result.push_str("nineteen"),
        20..=99 => {
            // Handle tens place (20-99)
            let tens = remainder / 10;
            let ones = remainder % 10;
            
            result.push_str(match tens {
                2 => "twenty",
                3 => "thirty",
                4 => "forty",
                5 => "fifty",
                6 => "sixty",
                7 => "seventy",
                8 => "eighty",
                9 => "ninety",
                _ => unreachable!(),
            });
            
            // Add hyphen and ones place if needed
            if ones > 0 {
                result.push_str("-");
                result.push_str(match ones {
                    1 => "one",
                    2 => "two",
                    3 => "three",
                    4 => "four",
                    5 => "five",
                    6 => "six",
                    7 => "seven",
                    8 => "eight",
                    9 => "nine",
                    _ => unreachable!(),
                });
            }
        },
        _ => unreachable!(),
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_short_numbers() {
        assert_eq!(negative_spell(0), String::from("zero"));
        assert_eq!(negative_spell(-1), String::from("minus one"));
        assert_eq!(negative_spell(-14), String::from("minus fourteen"));
        assert_eq!(negative_spell(-20), String::from("minus twenty"));
        assert_eq!(negative_spell(-22), String::from("minus twenty-two"));
        assert_eq!(negative_spell(-101), String::from("minus one hundred one"));
        assert_eq!(
            negative_spell(-120),
            String::from("minus one hundred twenty")
        );
        assert_eq!(
            negative_spell(-123),
            String::from("minus one hundred twenty-three")
        );
    }
    #[test]
    fn test_medium_numbers() {
        assert_eq!(negative_spell(-1000), String::from("minus one thousand"));
        assert_eq!(
            negative_spell(-1055),
            String::from("minus one thousand fifty-five")
        );
        assert_eq!(
            negative_spell(-1234),
            String::from("minus one thousand two hundred thirty-four")
        );
        assert_eq!(
            negative_spell(-10123),
            String::from("minus ten thousand one hundred twenty-three")
        );
    }
    #[test]
    fn test_long_numbers() {
        assert_eq!(
            negative_spell(-910112),
            String::from("minus nine hundred ten thousand one hundred twelve")
        );
        assert_eq!(
            negative_spell(-651123),
            String::from("minus six hundred fifty-one thousand one hundred twenty-three")
        );

        assert_eq!(
            negative_spell(-810000),
            String::from("minus eight hundred ten thousand")
        );
        assert_eq!(negative_spell(-1000000), String::from("minus one million"));
    }
    #[test]
    fn test_invalid_numbers() {
        assert_eq!(negative_spell(1), String::from("error: positive number"));
        assert_eq!(
            negative_spell(2390904),
            String::from("error: positive number")
        );
    }
}