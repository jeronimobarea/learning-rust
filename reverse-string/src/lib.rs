pub fn reverse(input: &str) -> String {
    let mut result = String::from("");
    _ = input.chars().rev().for_each(|c| result.push(c));
    result
}
