fn find_start(length: usize, data: &str) -> usize {
    for (index, window) in data
        .chars()
        .collect::<Vec<char>>()
        .windows(length)
        .enumerate()
    {
        let mut w_vec = window.to_vec();
        w_vec.sort();
        w_vec.dedup();
        if w_vec.len() == length {
            return index + length;
        }
    }

    0
}

pub fn find_start_of_packet(data: &str) -> usize {
    find_start(4, data)
}

pub fn find_start_of_message(data: &str) -> usize {
    find_start(14, data)
}
