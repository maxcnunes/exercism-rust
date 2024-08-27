pub fn is_armstrong_number(num: u32) -> bool {
    let str = num.to_string();
    let len: u32 = str.len().try_into().unwrap();

    str.chars()
        .map(|c| c.to_digit(10).unwrap().pow(len))
        .sum::<u32>()
        == num
}
