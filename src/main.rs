fn main() -> Result<(), std::io::Error> {
    println!("day 1 puzzle 1: {}", day1_puzzle1()?);
    println!("day 1 puzzle 2: {}", day1_puzzle2()?);
    Ok(())
}

#[inline(never)]
fn day1_puzzle1() -> Result<usize, std::io::Error> {
    Ok(std::fs::read_to_string("inputs/input-01")?
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()[..]
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count())
}

#[inline(never)]
fn day1_puzzle2() -> Result<usize, std::io::Error> {
    Ok(std::fs::read_to_string("inputs/input-01")?
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()[..]
        .windows(3)
        .map(|x| x.iter().sum())
        .collect::<Vec<i64>>()[..]
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count())
}
