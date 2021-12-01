fn main() -> Result<(), std::io::Error> {
    day1_puzzle1()?;
    day1_puzzle2()?;
    Ok(())
}

fn day1_puzzle1() -> Result<(), std::io::Error> {
    let count = std::fs::read_to_string("inputs/input-01")?
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()[..]
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count();
    println!("{}", count);
    Ok(())
}

fn day1_puzzle2() -> Result<(), std::io::Error> {
    let count = std::fs::read_to_string("inputs/input-01")?
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()[..]
        .windows(3)
        .map(|x| x.iter().sum())
        .collect::<Vec<i64>>()[..]
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count();
    println!("{}", count);
    Ok(())
}

