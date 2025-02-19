use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

type JsonInnerData = HashMap<String, HashMap<String, Vec<String>>>;

fn analyze_districts() -> JsonInnerData {
    let file = File::open("district.json");
    match file {
        Ok(_) => {}
        Err(e) => {
            println!("Error opening file: {}", e);
            return;
        }
    }
    let reader = BufReader::new(file.unwrap());
    let data: JsonInnerData = serde_json::from_reader(reader).unwrap();
    data
}
type JsonInnerData = HashMap<String, HashMap<String, Vec<String>>>;

fn find(a: &str, parent: &mut HashMap<String, String>) -> String {
    let mut a = a.to_string();
    while let Some(parent_a) = parent.get(&a) {
        if parent_a == &a {
            break;
        }
        a = parent_a.clone();
    }
    a
}

fn merge(a: &str, b: &str, parent: &mut HashMap<String, String>) {
    let pa = find(a, parent);
    let pb = find(b, parent);
    if pa != pb {
        parent.insert(pa, pb);
    }
}

fn dsu(data: &HashMap<String, Vec<String>>) -> i32 {
    // 建立并查集并返回集合数量
    let mut st = std::collections::HashSet::new();
    for (key, value) in data.iter() {
        st.insert(key);
        for i in value.iter() {
            st.insert(i);
        }
    }
    let mut parent = HashMap::new();
    for i in st.iter() {
        // 初始指向自己
        parent.insert((*i).clone(), (*i).clone());
    }
    for (key, value) in data.iter() {
        for i in value.iter() {
            merge(key, i, &mut parent);
        }
    }
    let mut result = std::collections::HashSet::new();
    for i in st.iter() {
        result.insert(find(i, &mut parent));
    }
    result.len() as i32
}

pub fn count_provinces() -> String {
    let data: JsonInnerData = analyze_districts();
    let mut result = Vec::new();

    for (key, value) in data.iter() {
        let siz: i32 = dsu(value);
        result.push(siz);
    }

    let mut res = String::new();
    for i in result.iter() {
        res.push_str(&i.to_string());
        res.push_str(",");
    }
    res.pop();
    res
}
