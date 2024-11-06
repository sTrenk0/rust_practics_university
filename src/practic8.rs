pub fn swap_case(input: &str) {
    let swapped: String = input
        .chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().to_string()
            } else {
                c.to_lowercase().to_string()
            }
        })
        .collect();

    println!("{}", swapped); // Выводим результат
}