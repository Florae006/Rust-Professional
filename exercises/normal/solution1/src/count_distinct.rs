pub fn new_count_distinct(input_str: &str) -> usize {
    let v: Vec<&str> = input_str.split(",").collect();
    let mut set = std::collections::HashSet::new();
    for s in v {
        set.insert(s);
    }
    set.len()
}
