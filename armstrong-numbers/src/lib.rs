pub fn is_armstrong_number(mut num: i32) -> bool {
    let str_num = num.to_string();
    let num_len: u32 = str_num.len().try_into().unwrap();

    str_num.chars().for_each(|n| {
        num -= i32::pow(n.to_digit(10).unwrap() as i32, num_len);
    });

    num == 0
}
