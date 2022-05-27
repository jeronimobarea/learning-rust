pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    if factors.len() == 0 {
        return 0;
    }

    (1..limit)
        .filter(|y| factors.iter().any(|x| x > &0 && y % x == 0))
        .sum()
}
