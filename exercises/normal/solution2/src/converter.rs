pub fn convert_base(num_str: &str, to_base: u32) -> String {
    let mut num = init_str(num_str);
    let mut res = String::new();
    while num > 0 {
        let mut c = num % to_base;
        if c < 10 {
            res.push((c as u8 + b'0') as char);
        } else {
            res.push((c as u8 - 10 + b'a') as char);
        }
        num /= to_base;
    }
    if res.len() == 0 {
        res.push('0');
    }
    res.chars().rev().collect()
}

pub fn init_str(s: &str) -> u32 {
    let mut v = s.split('(').collect::<Vec<&str>>();
    v[1] = &v[1][..v[1].len() - 1];
    let base = v[1].parse::<u32>().unwrap();
    let mut res = 0;
    for c in v[0].chars() {
        let mut num = c as u32;
        if num >= '0' as u32 && num <= '9' as u32 {
            num -= '0' as u32;
        } else {
            num = num - 'a' as u32 + 10;
        }
        res = res * base + num;
    }
    res
}
