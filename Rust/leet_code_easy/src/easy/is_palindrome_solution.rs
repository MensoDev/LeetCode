
pub fn run() {
    println!("EASY => IS PALINDROME NUMBER (121) => RESPONSE => || {:?} ||", is_palindrome(121));
    println!("EASY => IS PALINDROME NUMBER (14741) => RESPONSE => || {:?} ||", is_palindrome(14741));
    println!("EASY => IS PALINDROME NUMBER (3377) => RESPONSE => || {:?} ||", is_palindrome(3377));
}

fn is_palindrome(x: i32) -> bool {
    // Create a copy of the original value
    let original_value = x;
    let mut number = x;
    let mut inverted_number = 0;

    while number > 0 {
        
        let digit = number % 10;
        inverted_number = inverted_number * 10 + digit;
        number = number / 10;
    }

    original_value == inverted_number
}