use rand::Rng;

pub fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let total: u32 = shipments.iter().sum();
    let n = shipments.len() as u32;

    if total % n != 0 {
        return None;
    }

    let target = total / n;
    let mut moves = 0;
    let mut imbalance = 0;

    for &cargo in shipments {
        imbalance += cargo as i32 - target as i32;
        moves += imbalance.abs() as usize;
    }

    Some(moves as usize / 2)
}


pub fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let base: u32 = rng.gen_range(10..100);
    let remainder = rng.gen_range(0..n as u32);

    (0..n).map(|i| base + if i < remainder as usize { 1 } else { 0 }).collect()
}



