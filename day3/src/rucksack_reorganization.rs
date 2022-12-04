use std::collections::HashSet;

pub fn pair_line(line: &str) -> (&str, &str) {
    line.split_at(line.len() / 2)
}

pub fn find_commonality(comp1: &str, comp2: &str) -> String {
    let mut items: HashSet<char> = HashSet::new();

    for c1 in comp1.chars() {
        for c2 in comp2.chars() {
            if c1 == c2 {
                items.insert(c1);
            }
        }
    }

    items.iter().collect::<String>()
}

pub fn get_priority(items: &str) -> i32 {
    let mut total = 0;

    for item in items.chars() {
        if item.is_lowercase() {
            total += item as i32 - 96;
        } else if item.is_uppercase() {
            total += item as i32 - 38;
        }
    }

    total
}
