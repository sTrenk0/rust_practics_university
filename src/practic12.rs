use rand::Rng;

pub fn process_vector(n: usize) {
    let mut rng = rand::thread_rng();
    let vector: Vec<i32> = (0..n).map(|_| rng.gen_range(10..99)).collect();

    println!("Вектор: {:?}", vector);

    if vector.len() < 2 {
        println!("Вектор занадто малий для обчислення суми сусідніх елементів.");
        return;
    }

    let mut min_sum = i32::MAX;

    for i in 0..vector.len() - 1 {
        let pair_sum = vector[i] + vector[i + 1];
        if pair_sum < min_sum {
            min_sum = pair_sum;
        }
    }

    println!("Мінімальна сума сусідніх елементів: {}", min_sum);
}