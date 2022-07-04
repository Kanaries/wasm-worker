#![feature(slice_group_by)]
use std::collections::HashMap;

// no generic type parameter, no func
fn group_by(xs: Vec<(&str, i32)>) -> Vec<Vec<(String, i32)>> {
    let mut map:HashMap<&str, Vec<(String, i32)>> = HashMap::new();
    xs.iter().for_each(|(s, i)| {
        map.entry(s).or_insert(Vec::new()).push((s.to_string(), *i));
    });
    map.into_values().map(|v| v.into_iter().collect()).collect()
}

fn main() {
    println!("Hello, world!");
    let groupby_data = vec![
        ("abc", 0),
        ("edf", 1),
        ("lmn", 2),
        ("abc", 3),
        ("edf", 4),
        ("lmn", 5),
        ("abc", 6),
        ("zyx", 7),
        ("uer", 8),
    ];
    let res = group_by(groupby_data.clone());
    print!("{:?}", res);

    // 真是让人搞不懂
    /*
    ("abc", 0) ("edf", 1)
    (0, [("abc", 0)])
    ("edf", 1) ("lmn", 2)
    (1, [("edf", 1)])
    ("lmn", 2) ("abc", 3)
    (2, [("lmn", 2)])
    ("abc", 3) ("edf", 4)
    (3, [("abc", 3)])
    ("edf", 4) ("lmn", 5)
    (4, [("edf", 4)])
    ("lmn", 5) ("abc", 6)
    (5, [("lmn", 5)])
    ("abc", 6) ("zyx", 7)
    (6, [("abc", 6)])
    ("zyx", 7) ("uer", 8)
    (7, [("zyx", 7)])
    (8, [("uer", 8)])
    */
    groupby_data
        .group_by(|x, y| {
            println!("{:?} {:?}", x, y);
            x.0 == y.0
        })
        .enumerate()
        .for_each(|x| {
            println!("{:?}", x);
        })
}
