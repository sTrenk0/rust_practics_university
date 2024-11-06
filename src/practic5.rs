pub fn draw_envelope() {
    const WIDTH: usize = 11;
    const HEIGHT: usize = 7;
    let mut result = String::new();

    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            if i == 0 || i == HEIGHT - 1 || j == 0 || j == WIDTH - 1 || i == j || j == WIDTH - i - 1 {
                result.push('*');
            } else {
                result.push(' ');
            }
        }
        result.push('\n');
    }

    print!("{}", result);
}