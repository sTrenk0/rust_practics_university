use itertools::Itertools;

pub fn solve_puzzle() {
    let mut solutions = Vec::new();
    // Перебираємо всі можливі унікальні значення для 8 змінних від 1 до 8
    for perm in (1..=8).permutations(8) {
        let m = perm[0];
        let u = perm[1];
        let x = perm[2];
        let a = perm[3];
        let s = perm[4];
        let l = perm[5];
        let o = perm[6];
        let n = perm[7];

        // Формуємо числа "Муха" та "Слон"
        let muxa = 1000 * m + 100 * u + 10 * x + a;
        let slon = 1000 * s + 100 * l + 10 * o + n;

        // Перевіряємо, чи виконується умова
        if muxa * a == slon {
            solutions.push((muxa, a, slon));
        }
    }

    // Виводимо всі знайдені рішення
    for (muxa, a, slon) in &solutions {
        println!("{:>4}\n×    {}\n------\n{:>6}", muxa, a, slon);
    }

    // Виводимо кількість рішень
    println!("Задача має {} рішень", solutions.len());
}
