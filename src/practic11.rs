pub fn is_palindrome(num: i32) {
    let original = num.to_string();
    let reversed: String = original.chars().rev().collect();

    if original == reversed {
        println!("{} є паліндромом", num);
    } else {
        println!("{} не є паліндромом", num);
    }
}