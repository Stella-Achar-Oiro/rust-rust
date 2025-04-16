pub fn reverse_it(v: i32) -> String {
    // Convert the absolute value to a string
    let abs_str = v.abs().to_string();
    
    // Reverse the string
    let reversed: String = abs_str.chars().rev().collect();
    
    // Format according to requirements:
    // If negative, add a '-' at the beginning and then reversed+original
    // If positive, just return reversed+original
    if v < 0 {
        format!("-{}{}", reversed, v.abs())
    } else {
        format!("{}{}", reversed, v)
    }
}