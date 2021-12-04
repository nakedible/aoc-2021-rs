fn main() -> Result<(), std::io::Error> {
    println!("day 1 puzzle 1: {}", day1_puzzle1()?);
    println!("day 1 puzzle 2: {}", day1_puzzle2()?);
    println!("day 2 puzzle 1: {}", day2_puzzle1()?);
    println!("day 2 puzzle 2: {}", day2_puzzle2()?);
    println!("day 3 puzzle 1: {}", day3_puzzle1()?);
    println!("day 3 puzzle 2: {}", day3_puzzle2()?);
    println!("day 4 puzzle 1: {}", day4_puzzle1()?);
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
    let data = std::fs::read_to_string("inputs/input-03")?
        .lines()
        .map(|l| l.chars().map(|x| x.to_digit(2).unwrap() as i64).collect::<Vec<i64>>())
        .collect::<Vec<Vec<i64>>>();
    let datahalf = ((data.len() as i64)+1)/2;
    let mut sums = vec![0; data[0].len()];
    data.iter().for_each(|v| sums.iter_mut().zip(v).for_each(|(a, b)| *a += b));
    let gammar = sums.iter().map(|x| if x < &datahalf { 0 } else { 1 }).fold(0, |a, v| (a << 1) + v);
    let epsilonr = sums.iter().map(|x| if x < &datahalf { 1 } else { 0 }).fold(0, |a, v| (a << 1) + v);
    Ok((gammar * epsilonr) as usize)
}

fn day3_filter(data: &Vec<Vec<i64>>, most: bool) -> Vec<i64> {
    let mut curdata = data.clone();
    let (crita, critb) = if most { (0, 1) } else { (1, 0) };
    for i in 0..curdata[0].len() {
        let datahalf = ((curdata.len() as i64)+1)/2;
        let onebits = curdata.iter().fold(0, |a, v| a + v[i]);
        let criteria = if onebits < datahalf { crita } else { critb };
        curdata = curdata.iter().filter(|v| v[i] == criteria).cloned().collect::<Vec<Vec<i64>>>();
        if curdata.len() == 1 {
            break;
        }
    }
    curdata[0].clone()
}

#[inline(never)]
fn day3_puzzle2() -> Result<usize, std::io::Error> {
    let data = std::fs::read_to_string("inputs/input-03")?
        .lines()
        .map(|l| l.chars().map(|x| x.to_digit(2).unwrap() as i64).collect::<Vec<i64>>())
        .collect::<Vec<Vec<i64>>>();
    let oxygenr = day3_filter(&data, true).iter().fold(0, |a, v| (a << 1) + v);
    let co2r = day3_filter(&data, false).iter().fold(0, |a, v| (a << 1) + v);
    Ok((oxygenr * co2r) as usize)
}

#[inline(never)]
fn day4_puzzle1() -> Result<usize, std::io::Error> {
    let data = std::fs::read_to_string("inputs/input-03")?
        .lines();
    Ok(0 as usize)
}