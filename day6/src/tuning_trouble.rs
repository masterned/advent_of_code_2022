pub fn find_start_of_packet(data: &String) -> usize {
    for (index, window) in data.chars().collect::<Vec<char>>().windows(4).enumerate() {
        let mut w_vec = window.to_vec();
        w_vec.sort();
        w_vec.dedup();
        if w_vec.len() == 4 {
            return index + 4;
        }
    }

    0
}
