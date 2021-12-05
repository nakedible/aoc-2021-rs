fn main() -> Result<(), std::io::Error> {
    println!("day 1 puzzle 1: {}", day1_puzzle1()?);
    println!("day 1 puzzle 2: {}", day1_puzzle2()?);
    println!("day 2 puzzle 1: {}", day2_puzzle1()?);
    println!("day 2 puzzle 2: {}", day2_puzzle2()?);
    println!("day 3 puzzle 1: {}", day3_puzzle1()?);
    println!("day 3 puzzle 2: {}", day3_puzzle2()?);
    println!("day 4 puzzle 1: {}", day4_puzzle1()?);
    println!("day 4 puzzle 2: {}", day4_puzzle2()?);
    println!("day 5 puzzle 1: {}", day5_puzzle1()?);
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
        .map(|l| {
            l.chars()
                .map(|x| x.to_digit(2).unwrap() as i64)
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
    let datahalf = ((data.len() as i64) + 1) / 2;
    let mut sums = vec![0; data[0].len()];
    data.iter()
        .for_each(|v| sums.iter_mut().zip(v).for_each(|(a, b)| *a += b));
    let gammar = sums
        .iter()
        .map(|x| if x < &datahalf { 0 } else { 1 })
        .fold(0, |a, v| (a << 1) + v);
    let epsilonr = sums
        .iter()
        .map(|x| if x < &datahalf { 1 } else { 0 })
        .fold(0, |a, v| (a << 1) + v);
    Ok((gammar * epsilonr) as usize)
}

fn day3_filter(data: &Vec<Vec<i64>>, most: bool) -> Vec<i64> {
    let mut curdata = data.clone();
    let (crita, critb) = if most { (0, 1) } else { (1, 0) };
    for i in 0..curdata[0].len() {
        let datahalf = ((curdata.len() as i64) + 1) / 2;
        let onebits = curdata.iter().fold(0, |a, v| a + v[i]);
        let criteria = if onebits < datahalf { crita } else { critb };
        curdata = curdata
            .iter()
            .filter(|v| v[i] == criteria)
            .cloned()
            .collect::<Vec<Vec<i64>>>();
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
        .map(|l| {
            l.chars()
                .map(|x| x.to_digit(2).unwrap() as i64)
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<Vec<i64>>>();
    let oxygenr = day3_filter(&data, true).iter().fold(0, |a, v| (a << 1) + v);
    let co2r = day3_filter(&data, false)
        .iter()
        .fold(0, |a, v| (a << 1) + v);
    Ok((oxygenr * co2r) as usize)
}

fn day4_readboards() -> (Vec<i64>, Vec<[[i64; 5]; 5]>) {
    let file = std::fs::read_to_string("inputs/input-04").unwrap();
    let mut data = file.lines();
    let picks = data
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut boards = Vec::new();
    while data.next().is_some() {
        let mut board = [[-1i64; 5]; 5];
        for (_i, row) in board.iter_mut().enumerate() {
            let line = data.next().unwrap();
            for (j, col) in row.iter_mut().enumerate() {
                *col = line[j * 3..(j * 3) + 2]
                    .trim_start()
                    .parse::<i64>()
                    .unwrap();
            }
        }
        boards.push(board);
    }
    (picks, boards)
}

fn day4_markpick(board: &mut [[i64; 5]; 5], pick: i64) -> Option<(usize, usize)> {
    for (i, row) in board.iter_mut().enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            if *col == pick {
                *col = -1;
                return Some((i, j));
            }
        }
    }
    None
}

fn day4_checkbingo(board: &[[i64; 5]; 5], i: usize, j: usize) -> bool {
    board[i].iter().all(|&x| x == -1) || board.iter().map(|r| r[j]).all(|x| x == -1)
}

fn day4_calcscore(board: &[[i64; 5]; 5], pick: i64) -> i64 {
    let sums = board
        .iter()
        .map(|x| x.iter().filter(|&&y| y != -1).sum::<i64>())
        .sum::<i64>();
    sums * pick
}

#[inline(never)]
fn day4_puzzle1() -> Result<usize, std::io::Error> {
    let (picks, mut boards) = day4_readboards();

    for pick in picks {
        for board in boards.iter_mut() {
            if let Some((i, j)) = day4_markpick(board, pick) {
                if day4_checkbingo(board, i, j) {
                    let score = day4_calcscore(board, pick);
                    return Ok(score as usize);
                }
            }
        }
    }
    Ok(0 as usize)
}

fn day4_puzzle2() -> Result<usize, std::io::Error> {
    let (picks, mut boards) = day4_readboards();

    let mut last_score = -1;
    let mut left = boards.len() as i64;
    for pick in picks {
        for board in boards.iter_mut() {
            if let Some((i, j)) = day4_markpick(board, pick) {
                if day4_checkbingo(board, i, j) {
                    last_score = day4_calcscore(board, pick);
                    board
                        .iter_mut()
                        .for_each(|row| row.iter_mut().for_each(|col| *col = -1));
                    left -= 1;
                }
            }
        }
        if left == 0 {
            break;
        }
    }
    Ok(last_score as usize)
}

fn day5_puzzle1() -> Result<usize, std::io::Error> {
    let data = std::fs::read_to_string("inputs/input-05")?
        .lines()
        .map(|l| {
            let (a, b) = l.split_once(" -> ").unwrap();
            let (x1s, y1s) = a.split_once(",").unwrap();
            let (x2s, y2s) = b.split_once(",").unwrap();
            (
                (x1s.parse::<i64>().unwrap(), y1s.parse::<i64>().unwrap()),
                (x2s.parse::<i64>().unwrap(), y2s.parse::<i64>().unwrap()),
            )
        })
        .collect::<Vec<((i64, i64), (i64, i64))>>();
    let mut board = [[0; 1000]; 1000];
    for ((x1, y1), (x2, y2)) in data {
        if x1 == x2 {
            let (s, e) = if y2 < y1 { (y2, y1) } else { (y1, y2) };
            for i in s..e {
                board[i as usize][x1 as usize] += 1;
            }
        } else if y1 == y2 {
            let (s, e) = if x2 < x1 { (x2, x1) } else { (x1, x2) };
            for i in s..e {
                board[y1 as usize][i as usize] += 1;
            }
        }
    }
    Ok(0 as usize)
}
