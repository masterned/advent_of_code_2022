use std::collections::{HashMap, HashSet};

pub fn pair_line(line: &str) -> (&str, &str) {
    line.split_at(line.len() / 2)
}

fn remove_duplicate_items(items: &str) -> String {
    let mut result = HashSet::new();

    for item in items.chars() {
        result.insert(item);
    }

    result.iter().collect()
}

pub fn find_commonality(item_groups: &[&str]) -> String {
    let mut items: HashMap<char, usize> = HashMap::new();

    let group_count = item_groups.len();

    let item_groups = item_groups
        .iter()
        .map(|group| remove_duplicate_items(group));

    for item_group in item_groups {
        for item in item_group.chars() {
            items
                .entry(item)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }
    }

    items
        .iter()
        .filter(|(_, &count)| count == group_count)
        .map(|(item, _)| item)
        .collect()
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
