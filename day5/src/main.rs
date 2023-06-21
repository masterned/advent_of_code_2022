use std::fs;

use day5::supply_stacks::*;

fn main() -> std::io::Result<()> {
    let input = fs::read_to_string("./data/day5.txt")?;
    let lines: Vec<&str> = input.lines().collect();

    let mut separated_input = lines.split(|line| line.is_empty());

    let dock_spec = separated_input
        .next()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "Couldn't find dock spec"))?;
    let dock_spec = dock_spec.join("\n");
    // println!("{dock_spec:?}");

    let mut dock: Dock = dock_spec
        .parse()
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::Other, "Dock speck blew up!"))?;
    // println!("{dock:?}");

    let mut dock_duplicate = dock.clone(); // added this for Part 2

    let cmds_spec = separated_input.next().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::Other, "Couldn't find commands spec")
    })?;
    // println!("{cmds_spec:?}");

    let commands: Vec<Command> = cmds_spec
        .iter()
        .filter_map(|cmd_spec| cmd_spec.parse().ok())
        .collect();
    // println!("{commands:?}");

    commands.iter().for_each(|cmd| dock.mv(cmd));
    println!("Part 1: {}", dock.get_top_crates());

    commands.iter().for_each(|cmd| dock_duplicate.mv_9001(cmd));
    println!("Part 2: {}", dock_duplicate.get_top_crates());

    Ok(())
}
