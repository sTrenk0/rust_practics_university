pub fn draw_tree(levels: usize) {
    let mut result = String::new();

    (1..=levels).for_each(|level| {
        let rows = level + 1;

        (0..rows).for_each(|row| {
            result.extend(std::iter::repeat(' ').take(levels + rows - level - row - 1));
            result.extend(std::iter::repeat('*').take(2 * row + 1));
            result.push('\n');
        });
    });

    result.extend(std::iter::repeat(' ').take(levels - 1));
    result.push('*');
    result.push('\n');

    print!("{}", result);
}