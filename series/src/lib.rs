pub fn series(digits: &str, len: usize) -> Vec<String> {
    if digits.len() < len {
        return vec![];
    }
    if len <= 0 {
        return vec!["".to_string(); digits.len() + 1];
    }

    let digits_len = digits.len();
    if len == digits_len {
        return vec![digits.to_string()];
    }

    let mut res: Vec<String> = Vec::new();
    let mut prev = String::new();

    for (i, d) in digits.chars().enumerate() {
        if prev.is_empty() {
            prev = d.to_string();
            continue;
        }

        if i + 1 == digits_len {
            res.push(prev.clone());
            prev += &d.to_string();
            prev.remove(0);
            res.push(prev.clone());
            continue;
        }

        if prev.len() < len {
            prev += &d.to_string();
            continue;
        }

        res.push(prev.clone());
        prev.remove(0);
        prev += &d.to_string();
    }
    res
}
