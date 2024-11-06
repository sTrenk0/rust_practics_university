pub fn draw_diamond() {
    const SIZE: usize = 5;
    let mut result = String::new();

    for i in 0..SIZE {
        for _ in 0..(SIZE - i - 1) {
            result.push(' ');
        }
        for _ in 0..(2 * i + 1) {
            result.push('*');
        }
        result.push('\n');
    }

    for i in (0..SIZE - 1).rev() {
        for _ in 0..(SIZE - i - 1) {
            result.push(' ');
        }
        for _ in 0..(2 * i + 1) {
            result.push('*');
        }
        result.push('\n');
    }

    print!("{}", result);
}
