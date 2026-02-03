use std::collections::{HashMap, HashSet};


fn main() {
    let mut map = HashMap::new();

    match map.try_reserve(3) {
        Ok(_) => {}
        Err(e) => {
            println!("map reserve error: {e}");
            return;
        }
    }

    map.extend([
        ("a".to_string(), 10),
        ("b".to_string(), 20),
        ("c".to_string(), 30),
    ]);

    let map_clone = map.clone();

    map.retain(|_, v| *v >= 20);

    let mut opt_map = Some(map);

    let taken_map = match opt_map.take() {
        Some(m) => m,
        None => {
            println!("map take error");
            return;
        }
    };

    println!("{map_clone:?}");
    println!("{taken_map:?}");

    let mut set = HashSet::new();

    match set.try_reserve(3) {
        Ok(_) => {}
        Err(e) => {
            println!("set reserve error: {e}");
            return;
        }
    }

    set.extend([1, 2, 3]);

    let set_clone = set.clone();

    set.retain(|x| *x % 2 == 1);

    let mut opt_set = Some(set);

    let taken_set = match opt_set.take() {
        Some(s) => s,
        None => {
            println!("set take error");
            return;
        }
    };

    println!("{set_clone:?}");
    println!("{taken_set:?}");
}
