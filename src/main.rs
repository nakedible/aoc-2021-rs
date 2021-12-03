fn main() -> Result<(), std::io::Error> {
    println!("day 1 puzzle 1: {}", day1_puzzle1()?);
    println!("day 1 puzzle 2: {}", day1_puzzle2()?);
    println!("day 2 puzzle 1: {}", day2_puzzle1()?);
    println!("day 2 puzzle 2: {}", day2_puzzle2()?);
    println!("day 3 puzzle 1: {}", day3_puzzle1()?);
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

#[inline(never)]
fn day2_puzzle1() -> Result<usize, std::io::Error> {
    let (posh, depth) = std::fs::read_to_string("inputs/input-02")?
        .lines()
        .map(|x| {
            let mut parts = x.split_whitespace();
            (
                parts.next().unwrap(),
                parts.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .fold((0, 0), |(posh, depth), (dir, amount)| match dir {
            "forward" => (posh + amount, depth),
            "down" => (posh, depth + amount),
            "up" => (posh, depth - amount),
            _ => panic!("invalid input"),
        });
    Ok((posh * depth) as usize)
}

#[inline(never)]
fn day2_puzzle2() -> Result<usize, std::io::Error> {
    let (posh, depth, _aim) = std::fs::read_to_string("inputs/input-02")?
        .lines()
        .map(|x| {
            let mut parts = x.split_whitespace();
            (
                parts.next().unwrap(),
                parts.next().unwrap().parse::<i64>().unwrap(),
            )
        })
        .fold((0, 0, 0), |(posh, depth, aim), (dir, amount)| match dir {
            "forward" => (posh + amount, depth + (aim * amount), aim),
            "down" => (posh, depth, aim + amount),
            "up" => (posh, depth, aim - amount),
            _ => panic!("invalid input"),
        });
    Ok((posh * depth) as usize)
}

#[inline(never)]
fn day3_puzzle1() -> Result<usize, std::io::Error> {
    let _ = std::fs::read_to_string("inputs/input-03")?.lines().count();
    Ok(0 as usize)
}
