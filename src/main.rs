use cached::proc_macro::cached;
use pathfinding::directed::astar::astar;
use petgraph::graphmap::UnGraphMap;
use slice_group_by::GroupBy;
use std::cmp::{max, min};
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::VecDeque;

fn main() -> Result<(), std::io::Error> {
    println!("day 1 puzzle 1: {}", day1_puzzle1()?);
    println!("day 1 puzzle 2: {}", day1_puzzle2()?);
    println!("day 2 puzzle 1: {}", day2_puzzle1()?);
    println!("day 2 puzzle 2: {}", day2_puzzle2()?);
    println!("day 3 puzzle 1: {}", day3_puzzle1()?);
    println!("day 3 puzzle 2: {}", day3_puzzle2()?);
    println!("day 4 puzzle 1: {}", day4_puzzle1()?);
    println!("day 4 puzzle 2: {}", day4_puzzle2()?);
    println!("day 5 puzzle 1: {}", day5_puzzle12(false)?);
    println!("day 5 puzzle 2: {}", day5_puzzle12(true)?);
    println!("day 6 puzzle 1: {}", day6_puzzle1()?);
    println!("day 6 puzzle 2: {}", day6_puzzle2()?);
    println!("day 7 puzzle 1: {}", day7_puzzle1()?);
    println!("day 7 puzzle 2: {}", day7_puzzle2()?);
    println!("day 8 puzzle 1: {}", day8_puzzle1()?);
    println!("day 8 puzzle 2: {}", day8_puzzle2()?);
    println!("day 9 puzzle 1: {}", day9_puzzle1()?);
    println!("day 9 puzzle 2: {}", day9_puzzle2()?);
    println!("day 10 puzzle 1: {}", day10_puzzle1()?);
    println!("day 10 puzzle 2: {}", day10_puzzle2()?);
    println!("day 11 puzzle 1: {}", day11_puzzle1()?);
    println!("day 11 puzzle 2: {}", day11_puzzle2()?);
    println!("day 12 puzzle 1: {}", day12_puzzle1()?);
    println!("day 12 puzzle 2: {}", day12_puzzle2()?);
    println!("day 13 puzzle 1: {}", day13_puzzle1()?);
    println!("day 13 puzzle 2: {}", day13_puzzle2()?);
    println!("day 14 puzzle 1: {}", day14_puzzle1()?);
    println!("day 14 puzzle 2: {}", day14_puzzle2()?);
    println!("day 15 puzzle 1: {}", day15_puzzle1()?);
    println!("day 15 puzzle 2: {}", day15_puzzle2()?);
    println!("day 16 puzzle 1: {}", day16_puzzle1()?);
    println!("day 16 puzzle 2: {}", day16_puzzle2()?);
    println!("day 17 puzzle 1: {}", day17_puzzle1()?);
    println!("day 17 puzzle 2: {}", day17_puzzle2()?);
    println!("day 18 puzzle 1: {}", day18_puzzle1()?);
    println!("day 18 puzzle 2: {}", day18_puzzle2()?);
    println!("day 19 puzzle 1: {}", day19_puzzle1()?);
    println!("day 19 puzzle 2: {}", day19_puzzle2()?);
    println!("day 20 puzzle 1: {}", day20_puzzle1()?);
    println!("day 20 puzzle 2: {}", day20_puzzle2()?);
    println!("day 21 puzzle 1: {}", day21_puzzle1()?);
    println!("day 21 puzzle 2: {}", day21_puzzle2()?);
    println!("day 22 puzzle 1: {}", day22_puzzle1()?);
    println!("day 22 puzzle 2: {}", day22_puzzle2()?);
    println!("day 23 puzzle 1: {}", day23_puzzle1()?);
    println!("day 23 puzzle 2: {}", day23_puzzle2()?);
    println!("day 24 puzzle 1: {}", day24_puzzle1()?);
    println!("day 24 puzzle 2: {}", day24_puzzle2()?);
    Ok(())
}

#[inline(never)]
pub fn day1_puzzle1() -> Result<usize, std::io::Error> {
    Ok(std::fs::read_to_string("inputs/input-01")?
        .lines()
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>()[..]
        .windows(2)
        .filter(|x| x[0] < x[1])
        .count())
}

#[inline(never)]
pub fn day1_puzzle2() -> Result<usize, std::io::Error> {
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
pub fn day2_puzzle1() -> Result<usize, std::io::Error> {
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
pub fn day2_puzzle2() -> Result<usize, std::io::Error> {
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
pub fn day3_puzzle1() -> Result<usize, std::io::Error> {
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
pub fn day3_puzzle2() -> Result<usize, std::io::Error> {
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
pub fn day4_puzzle1() -> Result<usize, std::io::Error> {
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

#[inline(never)]
pub fn day4_puzzle2() -> Result<usize, std::io::Error> {
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

#[inline(never)]
pub fn day5_puzzle12(diag: bool) -> Result<usize, std::io::Error> {
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
    let mut board = [[0i64; 1000]; 1000];
    for ((x1, y1), (x2, y2)) in data {
        if x1 == x2 {
            let (s, e) = if y2 < y1 { (y2, y1) } else { (y1, y2) };
            for i in s..=e {
                board[i as usize][x1 as usize] += 1;
            }
        } else if y1 == y2 {
            let (s, e) = if x2 < x1 { (x2, x1) } else { (x1, x2) };
            for i in s..=e {
                board[y1 as usize][i as usize] += 1;
            }
        } else if diag && (x2 - x1).abs() == (y2 - y1).abs() {
            let xi = if x2 < x1 { -1 } else { 1 };
            let yi = if y2 < y1 { -1 } else { 1 };
            let (mut cx, mut cy) = (x1, y1);
            loop {
                board[cy as usize][cx as usize] += 1;
                if (cx, cy) == (x2, y2) {
                    break;
                }
                cx += xi;
                cy += yi;
            }
        }
    }
    let result = board.iter().fold(0i64, |a, row| {
        a + row
            .iter()
            .fold(0i64, |a, col| if *col >= 2 { a + 1 } else { a })
    });
    Ok(result as usize)
}

#[inline(never)]
pub fn day6_puzzle1() -> Result<usize, std::io::Error> {
    let mut fish = std::fs::read_to_string("inputs/input-06")?
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    for _ in 0..80 {
        let mut spawns = 0;
        for f in fish.iter_mut() {
            *f -= 1;
            if *f < 0 {
                *f += 7;
                spawns += 1;
            }
        }
        for _ in 0..spawns {
            fish.push(8);
        }
    }
    Ok(fish.len() as usize)
}

#[inline(never)]
pub fn day6_puzzle2() -> Result<usize, std::io::Error> {
    let fish = std::fs::read_to_string("inputs/input-06")?
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut fishdays = [0; 9];
    fish.iter().for_each(|&x| fishdays[x as usize] += 1);
    for _ in 0..256 {
        let spawns = fishdays[0];
        for day in 1..=8 {
            fishdays[day - 1] = fishdays[day];
        }
        fishdays[6] += spawns;
        fishdays[8] = spawns;
    }
    let total: i64 = fishdays.iter().sum();
    Ok(total as usize)
}

#[inline(never)]
pub fn day7_puzzle1() -> Result<usize, std::io::Error> {
    let crabs = std::fs::read_to_string("inputs/input-07")?
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let minfuel = (0..2000)
        .map(|i| crabs.iter().map(|x| (x - i).abs()).sum::<i64>())
        .min()
        .unwrap();
    Ok(minfuel as usize)
}

#[inline(never)]
pub fn day7_puzzle2() -> Result<usize, std::io::Error> {
    let crabs = std::fs::read_to_string("inputs/input-07")?
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let minfuel = (0..=2000)
        .map(|i| {
            crabs
                .iter()
                .map(|x| {
                    let dist = (x - i).abs();
                    (1..=dist).sum::<i64>()
                })
                .sum::<i64>()
        })
        .min()
        .unwrap();
    Ok(minfuel as usize)
}

#[inline(never)]
pub fn day8_puzzle1() -> Result<usize, std::io::Error> {
    let data = std::fs::read_to_string("inputs/input-08")?
        .lines()
        .map(|l| {
            l.split(" | ")
                .map(|p| p.split(" ").map(str::to_owned).collect::<Vec<String>>())
                .collect::<Vec<Vec<String>>>()
        })
        .collect::<Vec<Vec<Vec<String>>>>();
    let mut count = 0;
    for row in &data {
        for dig in &row[1] {
            match dig.len() {
                2 | 3 | 4 | 7 => {
                    count += 1;
                }
                _ => {}
            }
        }
    }
    Ok(count as usize)
}

fn day8_ag_to_u8(a: &str) -> u8 {
    a.chars()
        .map(|x| match x {
            'a' => 1,
            'b' => 2,
            'c' => 4,
            'd' => 8,
            'e' => 16,
            'f' => 32,
            'g' => 64,
            _ => 127,
        })
        .fold(0u8, |a, b| a | b)
}

fn day8_decode_digits(digits: &Vec<u8>) -> Vec<u8> {
    let mut sorted = digits.clone();
    sorted.sort_by_key(|a| a.count_ones());
    if let [one, seven, four, fives1, fives2, fives3, sixes1, sixes2, sixes3, eight] = sorted[..] {
        let fournot = four ^ one;
        let (three, twofive1, twofive2) = if fives1 & seven == seven {
            (fives1, fives2, fives3)
        } else if fives2 & seven == seven {
            (fives2, fives1, fives3)
        } else if fives3 & seven == seven {
            (fives3, fives1, fives2)
        } else {
            panic!("invalid input");
        };
        let (two, five) = if twofive1 & fournot == fournot {
            (twofive2, twofive1)
        } else if twofive2 & fournot == fournot {
            (twofive1, twofive2)
        } else {
            panic!("invalid input");
        };
        let (nine, zerosix1, zerosix2) = if sixes1 & four == four {
            (sixes1, sixes2, sixes3)
        } else if sixes2 & four == four {
            (sixes2, sixes1, sixes3)
        } else if sixes3 & four == four {
            (sixes3, sixes1, sixes2)
        } else {
            panic!("invalid input");
        };
        let (zero, six) = if zerosix1 & one == one {
            (zerosix1, zerosix2)
        } else if zerosix2 & one == one {
            (zerosix2, zerosix1)
        } else {
            panic!("invalid input");
        };
        return vec![zero, one, two, three, four, five, six, seven, eight, nine];
    } else {
        panic!("invalid input");
    };
}

fn day8_decode_digit(digits: &Vec<u8>, val: u8) -> i64 {
    digits.iter().position(|&x| x == val).unwrap() as i64
}

#[inline(never)]
pub fn day8_puzzle2() -> Result<usize, std::io::Error> {
    let data = std::fs::read_to_string("inputs/input-08")?
        .lines()
        .map(|l| {
            let (digits, vals) = l.split_once(" | ").unwrap();
            (
                digits.split(" ").map(day8_ag_to_u8).collect::<Vec<u8>>(),
                vals.split(" ").map(day8_ag_to_u8).collect::<Vec<u8>>(),
            )
        })
        .collect::<Vec<(Vec<u8>, Vec<u8>)>>();
    let mut sum = 0;
    for row in &data {
        let digits = day8_decode_digits(&row.0);
        let value = row
            .1
            .iter()
            .map(|&x| day8_decode_digit(&digits, x))
            .fold(0, |a, v| a * 10 + v);
        sum += value;
    }
    Ok(sum as usize)
}

fn day9_safeget(heightmap: &[[i8; 100]; 100], row: i64, col: i64) -> i8 {
    if row < 0 || row > 99 || col < 0 || col > 99 {
        127
    } else {
        heightmap[row as usize][col as usize]
    }
}

#[inline(never)]
pub fn day9_puzzle1() -> Result<usize, std::io::Error> {
    let mut heightmap = [[0i8; 100]; 100];
    std::fs::read_to_string("inputs/input-09")?
        .lines()
        .enumerate()
        .for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, c)| {
                heightmap[row][col] = c.to_digit(10).unwrap() as i8;
            })
        });
    let mut risks: i64 = 0;
    for row in 0..100 {
        for col in 0..100 {
            let point = day9_safeget(&heightmap, row, col);
            if point < day9_safeget(&heightmap, row + 1, col)
                && point < day9_safeget(&heightmap, row - 1, col)
                && point < day9_safeget(&heightmap, row, col + 1)
                && point < day9_safeget(&heightmap, row, col - 1)
            {
                //println!("low point! {} {} {}", row, col, point);
                risks += (point + 1) as i64;
            }
        }
    }
    Ok(risks as usize)
}

fn day9_basin_size(heightmap: &mut [[i8; 100]; 100], row: i64, col: i64) -> i64 {
    if day9_safeget(heightmap, row, col) < 9 {
        heightmap[row as usize][col as usize] = 9;
        1 + day9_basin_size(heightmap, row + 1, col)
            + day9_basin_size(heightmap, row - 1, col)
            + day9_basin_size(heightmap, row, col + 1)
            + day9_basin_size(heightmap, row, col - 1)
    } else {
        0
    }
}

#[inline(never)]
pub fn day9_puzzle2() -> Result<usize, std::io::Error> {
    let mut heightmap = [[0i8; 100]; 100];
    std::fs::read_to_string("inputs/input-09")?
        .lines()
        .enumerate()
        .for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, c)| {
                heightmap[row][col] = c.to_digit(10).unwrap() as i8;
            })
        });
    let mut basins: Vec<(i64, i64)> = Vec::new();
    for row in 0..100 {
        for col in 0..100 {
            let point = day9_safeget(&heightmap, row, col);
            if point < day9_safeget(&heightmap, row + 1, col)
                && point < day9_safeget(&heightmap, row - 1, col)
                && point < day9_safeget(&heightmap, row, col + 1)
                && point < day9_safeget(&heightmap, row, col - 1)
            {
                basins.push((row, col));
            }
        }
    }
    let mut basinsizes = basins
        .iter()
        .map(|(row, col)| day9_basin_size(&mut heightmap, *row, *col))
        .collect::<Vec<i64>>();
    basinsizes.sort();
    basinsizes.reverse();
    let solution = basinsizes[0..3].iter().fold(1, |a, b| a * b);
    Ok(solution as usize)
}

fn day10_matches(a: char, b: Option<char>) -> bool {
    match (a, b) {
        (')', Some('(')) => true,
        (']', Some('[')) => true,
        ('}', Some('{')) => true,
        ('>', Some('<')) => true,
        _ => false,
    }
}

fn day10_value(ch: char) -> i64 {
    match ch {
        ')' => 3,
        ']' => 57,
        '}' => 1197,
        '>' => 25137,
        _ => panic!("invalid input"),
    }
}

#[inline(never)]
pub fn day10_puzzle1() -> Result<usize, std::io::Error> {
    let lines = std::fs::read_to_string("inputs/input-10")?
        .lines()
        .map(str::to_owned)
        .collect::<Vec<String>>();
    let mut total: i64 = 0;
    for line in lines {
        let mut stack = Vec::with_capacity(100);
        for ch in line.chars() {
            match ch {
                '(' | '[' | '{' | '<' => {
                    stack.push(ch);
                }
                ')' | ']' | '}' | '>' => {
                    if day10_matches(ch, stack.pop()) {
                    } else {
                        total += day10_value(ch);
                        break;
                    }
                }
                _ => panic!("invalid input"),
            }
        }
    }
    Ok(total as usize)
}

fn day10_score(ch: char) -> i64 {
    match ch {
        '(' => 1,
        '[' => 2,
        '{' => 3,
        '<' => 4,
        _ => panic!("invalid input"),
    }
}

#[inline(never)]
pub fn day10_puzzle2() -> Result<usize, std::io::Error> {
    let lines = std::fs::read_to_string("inputs/input-10")?
        .lines()
        .map(str::to_owned)
        .collect::<Vec<String>>();
    let mut scores: Vec<i64> = Vec::new();
    for line in lines {
        let mut stack = Vec::with_capacity(100);
        let mut error = false;
        for ch in line.chars() {
            match ch {
                '(' | '[' | '{' | '<' => {
                    stack.push(ch);
                }
                ')' | ']' | '}' | '>' => {
                    if day10_matches(ch, stack.pop()) {
                    } else {
                        error = true;
                        break;
                    }
                }
                _ => panic!("invalid input"),
            }
        }
        if error {
            continue;
        }
        stack.reverse();
        scores.push(stack.iter().fold(0, |a, &ch| (a * 5) + day10_score(ch)));
    }
    scores.sort();
    let middle = scores[(scores.len() / 2)];
    Ok(middle as usize)
}

#[inline(never)]
pub fn day11_puzzle1() -> Result<usize, std::io::Error> {
    let mut heightmap = [[0i8; 10]; 10];
    std::fs::read_to_string("inputs/input-11")?
        .lines()
        .enumerate()
        .for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, c)| {
                heightmap[row][col] = c.to_digit(10).unwrap() as i8;
            })
        });
    let mut flashq = VecDeque::with_capacity(100);
    let mut flashes = 0i64;
    for _ in 0..100 {
        for row in 0..10 {
            for col in 0..10 {
                heightmap[row][col] += 1;
                if heightmap[row][col] == 10 {
                    flashes += 1;
                    flashq.push_back((row as i64, col as i64));
                }
            }
        }
        while let Some((crow, ccol)) = flashq.pop_front() {
            heightmap[crow as usize][ccol as usize] = 0;
            for row in max(crow - 1, 0)..=min(crow + 1, 9) {
                for col in max(ccol - 1, 0)..=min(ccol + 1, 9) {
                    if heightmap[row as usize][col as usize] == 0 {
                        continue;
                    }
                    heightmap[row as usize][col as usize] += 1;
                    if heightmap[row as usize][col as usize] == 10 {
                        flashes += 1;
                        flashq.push_back((row as i64, col as i64));
                    }
                }
            }
        }
    }
    Ok(flashes as usize)
}

#[inline(never)]
pub fn day11_puzzle2() -> Result<usize, std::io::Error> {
    let mut heightmap = [[0i8; 10]; 10];
    std::fs::read_to_string("inputs/input-11")?
        .lines()
        .enumerate()
        .for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, c)| {
                heightmap[row][col] = c.to_digit(10).unwrap() as i8;
            })
        });
    let mut flashq = VecDeque::with_capacity(100);
    let mut steps = 0i64;
    loop {
        steps += 1;
        let mut flashes = 0i64;
        for row in 0..10 {
            for col in 0..10 {
                heightmap[row][col] += 1;
                if heightmap[row][col] == 10 {
                    flashes += 1;
                    flashq.push_back((row as i64, col as i64));
                }
            }
        }
        while let Some((crow, ccol)) = flashq.pop_front() {
            heightmap[crow as usize][ccol as usize] = 0;
            for row in max(crow - 1, 0)..=min(crow + 1, 9) {
                for col in max(ccol - 1, 0)..=min(ccol + 1, 9) {
                    if heightmap[row as usize][col as usize] == 0 {
                        continue;
                    }
                    heightmap[row as usize][col as usize] += 1;
                    if heightmap[row as usize][col as usize] == 10 {
                        flashes += 1;
                        flashq.push_back((row as i64, col as i64));
                    }
                }
            }
        }
        if flashes == 100 {
            break;
        }
    }
    Ok(steps as usize)
}

fn day12_countpaths<'a>(graph: &UnGraphMap<&'a str, ()>, visited: &mut Vec<&'a str>) -> i64 {
    let mut ret = 0;
    for neigh in graph.neighbors(visited[visited.len() - 1]) {
        if neigh == "end" {
            ret += 1;
        } else if neigh.chars().all(|c| matches!(c, 'a'..='z')) && visited.contains(&neigh) {
        } else {
            visited.push(neigh);
            ret += day12_countpaths(graph, visited);
            visited.pop();
        }
    }
    ret
}

#[inline(never)]
pub fn day12_puzzle1() -> Result<usize, std::io::Error> {
    let mut graph = UnGraphMap::new();
    let file = std::fs::read_to_string("inputs/input-12")?;
    file.lines().for_each(|line| {
        let (a, b) = line.split_once("-").unwrap();
        graph.add_edge(a, b, ());
    });
    let mut visited = Vec::new();
    visited.push("start");
    let answer = day12_countpaths(&graph, &mut visited);
    Ok(answer as usize)
}

fn day12_countpaths2<'a>(
    graph: &UnGraphMap<&'a str, ()>,
    visited: &mut Vec<&'a str>,
    double: bool,
) -> i64 {
    let mut ret = 0;
    for neigh in graph.neighbors(visited[visited.len() - 1]) {
        if neigh == "end" {
            ret += 1;
        } else if neigh == "start" {
            continue;
        } else {
            let mut dubbel = double;
            if neigh.chars().all(|c| matches!(c, 'a'..='z')) && visited.contains(&neigh) {
                if double {
                    continue;
                } else {
                    dubbel = true;
                }
            }
            visited.push(neigh);
            ret += day12_countpaths2(graph, visited, dubbel);
            visited.pop();
        }
    }
    ret
}

#[inline(never)]
pub fn day12_puzzle2() -> Result<usize, std::io::Error> {
    let mut graph = UnGraphMap::new();
    let file = std::fs::read_to_string("inputs/input-12")?;
    file.lines().for_each(|line| {
        let (a, b) = line.split_once("-").unwrap();
        graph.add_edge(a, b, ());
    });
    let mut visited = Vec::new();
    visited.push("start");
    let answer = day12_countpaths2(&graph, &mut visited, false);
    Ok(answer as usize)
}

fn day13_fold(points: &mut Vec<(i64, i64)>, x: bool, val: i64) {
    for p in points {
        if x {
            if p.0 < val {
                continue;
            } else {
                *p = (val - (p.0 - val), p.1);
            }
        } else {
            if p.1 < val {
                continue;
            } else {
                *p = (p.0, val - (p.1 - val));
            }
        }
    }
}

#[inline(never)]
pub fn day13_puzzle1() -> Result<usize, std::io::Error> {
    let file = std::fs::read_to_string("inputs/input-13")?;
    let (points, splits) = file.split_once("\n\n").unwrap();
    let mut points = points
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
        })
        .collect::<Vec<(i64, i64)>>();
    let splits = splits
        .lines()
        .map(|l| {
            if !l.starts_with("fold along ") {
                panic!("invalid input");
            }
            let (dir, val) = l[11..].split_once("=").unwrap();
            (dir == "x", val.parse::<i64>().unwrap())
        })
        .collect::<Vec<(bool, i64)>>();
    let (x, val) = splits.iter().next().unwrap();
    day13_fold(&mut points, *x, *val);
    points.sort();
    points.dedup();
    Ok(points.len() as usize)
}

#[inline(never)]
pub fn day13_puzzle2() -> Result<usize, std::io::Error> {
    let file = std::fs::read_to_string("inputs/input-13")?;
    let (points, splits) = file.split_once("\n\n").unwrap();
    let mut points = points
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(",").unwrap();
            (x.parse::<i64>().unwrap(), y.parse::<i64>().unwrap())
        })
        .collect::<Vec<(i64, i64)>>();
    let splits = splits
        .lines()
        .map(|l| {
            if !l.starts_with("fold along ") {
                panic!("invalid input");
            }
            let (dir, val) = l[11..].split_once("=").unwrap();
            (dir == "x", val.parse::<i64>().unwrap())
        })
        .collect::<Vec<(bool, i64)>>();
    for (x, val) in splits {
        day13_fold(&mut points, x, val);
        points.sort();
        points.dedup();
    }
    let mut output = [[false; 40]; 6];
    for (x, y) in points {
        output[y as usize][x as usize] = true;
    }
    for row in output {
        for col in row {
            print!("{}", if col { "*" } else { " " });
        }
        println!("");
    }
    Ok(0 as usize)
}

#[inline(never)]
pub fn day14_puzzle1() -> Result<usize, std::io::Error> {
    let file = std::fs::read_to_string("inputs/input-14")?;
    let (start, rules) = file.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|l| {
            let (s, f) = l.split_once(" -> ").unwrap();
            (s.as_bytes(), f.as_bytes()[0])
        })
        .collect::<HashMap<&[u8], u8>>();
    let mut cur = start.as_bytes().to_owned();
    for _ in 0..10 {
        let mut next = Vec::with_capacity((cur.len() * 2) - 1);
        next.push(cur[0]);
        for s in cur.windows(2) {
            next.push(*rules.get(s).unwrap());
            next.push(s[1]);
        }
        cur = next;
    }
    let mut counts = HashMap::new();
    for c in &cur {
        *counts.entry(c).or_insert(0) += 1;
    }
    let response = *counts.values().max().unwrap() - *counts.values().min().unwrap();
    Ok(response as usize)
}

#[inline(never)]
pub fn day14_puzzle2() -> Result<usize, std::io::Error> {
    let file = std::fs::read_to_string("inputs/input-14")?;
    let (start, rules) = file.split_once("\n\n").unwrap();
    let rules = rules
        .lines()
        .map(|l| {
            let (s, f) = l.split_once(" -> ").unwrap();
            (
                s.as_bytes()[0..2].try_into().expect("invalid input"),
                f.as_bytes()[0],
            )
        })
        .collect::<HashMap<[u8; 2], u8>>();
    let mut pairs: HashMap<[u8; 2], i64> = HashMap::new();
    for s in start.as_bytes().windows(2) {
        *pairs
            .entry(s[0..2].try_into().expect("invalid input"))
            .or_insert(0) += 1;
    }
    for _ in 0..40 {
        let mut next: HashMap<[u8; 2], i64> = HashMap::new();
        for (pair, count) in pairs {
            let add = rules.get(&pair).unwrap();
            *next.entry([pair[0], *add]).or_insert(0) += count;
            *next.entry([*add, pair[1]]).or_insert(0) += count;
        }
        pairs = next;
    }
    let mut counts = HashMap::new();
    counts.insert(start.as_bytes()[0], 1);
    for (pair, count) in pairs {
        *counts.entry(pair[1]).or_insert(0) += count;
    }
    let response = *counts.values().max().unwrap() - *counts.values().min().unwrap();
    Ok(response as usize)
}

fn day15_neighbours<const L: usize>(
    map: &[[i8; L]; L],
    pos: &(i64, i64),
) -> Vec<((i64, i64), i64)> {
    let mut ret = Vec::new();
    for (row, col) in [
        (pos.1 - 1, pos.0),
        (pos.1 + 1, pos.0),
        (pos.1, pos.0 - 1),
        (pos.1, pos.0 + 1),
    ] {
        if row < 0 || row >= map.len() as i64 || col < 0 || col >= map[0].len() as i64 {
            continue;
        }
        ret.push(((col, row), map[row as usize][col as usize] as i64));
    }
    ret
}

#[inline(never)]
pub fn day15_puzzle1() -> Result<usize, std::io::Error> {
    let mut dangermap = [[0i8; 100]; 100];
    std::fs::read_to_string("inputs/input-15")?
        .lines()
        .enumerate()
        .for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, c)| {
                dangermap[row][col] = c.to_digit(10).unwrap() as i8;
            })
        });
    let (_path, cost) = astar(
        &(0i64, 0i64),
        |p| day15_neighbours(&dangermap, p),
        |(x, y)| i64::abs(99 - x) + i64::abs(99 - y),
        |p| *p == (99, 99),
    )
    .unwrap();
    Ok(cost as usize)
}

fn day15_dupmap(
    map: &mut [[i8; 500]; 500],
    srcreprow: i64,
    srcrepcol: i64,
    dstreprow: i64,
    dstrepcol: i64,
) {
    for row in 0..100 {
        for col in 0..100 {
            map[(dstreprow * 100 + row) as usize][(dstrepcol * 100 + col) as usize] =
                if map[(srcreprow * 100 + row) as usize][(srcrepcol * 100 + col) as usize] == 9 {
                    1
                } else {
                    map[(srcreprow * 100 + row) as usize][(srcrepcol * 100 + col) as usize] + 1
                };
        }
    }
}

#[inline(never)]
pub fn day15_puzzle2() -> Result<usize, std::io::Error> {
    let mut dangermap = [[0i8; 500]; 500];
    std::fs::read_to_string("inputs/input-15")?
        .lines()
        .enumerate()
        .for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, c)| {
                dangermap[row][col] = c.to_digit(10).unwrap() as i8;
            })
        });
    for reprow in 1..5 {
        day15_dupmap(&mut dangermap, reprow - 1, 0, reprow, 0);
    }
    for reprow in 0..5 {
        for repcol in 1..5 {
            day15_dupmap(&mut dangermap, reprow, repcol - 1, reprow, repcol);
        }
    }
    let (_path, cost) = astar(
        &(0i64, 0i64),
        |p| day15_neighbours(&dangermap, p),
        |(x, y)| i64::abs(499 - x) + i64::abs(499 - y),
        |p| *p == (499, 499),
    )
    .unwrap();
    Ok(cost as usize)
}

fn day16_readnum(it: &mut impl Iterator<Item = char>, n: usize) -> Option<i64> {
    let num = it.take(n).collect::<String>();
    if num.len() == n {
        Some(i64::from_str_radix(&num, 2).unwrap())
    } else {
        None
    }
}

#[inline(never)]
pub fn day16_puzzle1() -> Result<usize, std::io::Error> {
    let bits = std::fs::read_to_string("inputs/input-16")?
        .chars()
        .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
        .collect::<String>();
    let mut chars = bits.chars();
    let mut vertotal = 0;
    loop {
        let version = day16_readnum(&mut chars, 3);
        if version.is_none() {
            break;
        }
        vertotal += version.unwrap();
        let typ = day16_readnum(&mut chars, 3);
        if typ.is_none() {
            break;
        }
        if typ.unwrap() == 4 {
            loop {
                let num = day16_readnum(&mut chars, 5).unwrap();
                if num & 16 == 0 {
                    break;
                }
            }
        } else {
            if day16_readnum(&mut chars, 1).unwrap() == 0 {
                let _ = day16_readnum(&mut chars, 15).unwrap();
            } else {
                let _ = day16_readnum(&mut chars, 11).unwrap();
            }
        }
    }
    Ok(vertotal as usize)
}

fn day16_parsesub(it: &mut impl Iterator<Item = char>) -> Vec<i64> {
    let mut ret = Vec::new();
    if day16_readnum(it, 1).unwrap() == 0 {
        let len = day16_readnum(it, 15).unwrap();
        let substr = it.take(len as usize).collect::<String>();
        let mut subs = substr.chars();
        loop {
            if let Some(val) = day16_compute(&mut subs) {
                ret.push(val);
            } else {
                break;
            }
        }
    } else {
        let num = day16_readnum(it, 11).unwrap();
        for _ in 0..num {
            ret.push(day16_compute(it).unwrap());
        }
    }
    ret
}

fn day16_parsenum(it: &mut impl Iterator<Item = char>) -> i64 {
    let mut ret = 0;
    loop {
        let num = day16_readnum(it, 5).unwrap();
        ret = (ret << 4) | (num & 0xf);
        if num & 16 == 0 {
            break;
        }
    }
    ret
}

fn day16_compute(it: &mut impl Iterator<Item = char>) -> Option<i64> {
    let version = day16_readnum(it, 3);
    if version.is_none() {
        return None;
    }
    let typ = day16_readnum(it, 3);
    if typ.is_none() {
        return None;
    }
    let ret = match typ.unwrap() {
        0 => day16_parsesub(it).iter().sum::<i64>(),
        1 => day16_parsesub(it).iter().product::<i64>(),
        2 => *day16_parsesub(it).iter().min().unwrap(),
        3 => *day16_parsesub(it).iter().max().unwrap(),
        4 => day16_parsenum(it),
        5 => {
            if let [a, b] = day16_parsesub(it)[..] {
                if a > b {
                    1
                } else {
                    0
                }
            } else {
                panic!("invalid input")
            }
        }
        6 => {
            if let [a, b] = day16_parsesub(it)[..] {
                if a < b {
                    1
                } else {
                    0
                }
            } else {
                panic!("invalid input")
            }
        }
        7 => {
            if let [a, b] = day16_parsesub(it)[..] {
                if a == b {
                    1
                } else {
                    0
                }
            } else {
                panic!("invalid input")
            }
        }
        _ => panic!("invalid input"),
    };
    Some(ret)
}

#[inline(never)]
pub fn day16_puzzle2() -> Result<usize, std::io::Error> {
    let bits = std::fs::read_to_string("inputs/input-16")?
        .chars()
        .map(|c| format!("{:04b}", c.to_digit(16).unwrap()))
        .collect::<String>();
    let mut chars = bits.chars();
    let ret = day16_compute(&mut chars).unwrap();
    Ok(ret as usize)
}

fn day17_xvals(xmin: i64, xmax: i64) -> Vec<i64> {
    let mut ret = Vec::new();
    for v in 1..=xmax + 1 {
        let mut xpos = 0;
        let mut xvel = v;
        loop {
            xpos += xvel;
            if xpos > xmax {
                break;
            } else if xpos >= xmin {
                ret.push(v);
                break;
            } else if xvel == 1 {
                break;
            }
            xvel -= 1;
        }
    }
    ret
}

fn day17_hits(xvel: i64, yvel: i64, xmin: i64, ymin: i64, xmax: i64, ymax: i64) -> Option<i64> {
    let (mut xpos, mut ypos) = (0i64, 0i64);
    let (mut xvel, mut yvel) = (xvel, yvel);
    let mut yposmax = 0;
    for _ in 0..1000 {
        xpos += xvel;
        ypos += yvel;
        if xvel > 0 {
            xvel -= 1;
        }
        yvel -= 1;
        if ypos > yposmax {
            yposmax = ypos;
        }
        if xpos >= xmin && xpos <= xmax && ypos >= ymin && ypos <= ymax {
            return Some(yposmax);
        } else if xpos > xmax || ypos < ymin {
            return None;
        }
    }
    panic!("should not get here")
}

#[inline(never)]
pub fn day17_puzzle1() -> Result<usize, std::io::Error> {
    let data = std::fs::read_to_string("inputs/input-17")?;
    let (xs, ys) = data.split_once(", ").unwrap();
    let (_, xs) = xs.split_once("=").unwrap();
    let (_, ys) = ys.split_once("=").unwrap();
    let (xmin, xmax) = xs.split_once("..").unwrap();
    let (ymin, ymax) = ys.split_once("..").unwrap();
    let (xmin, xmax): (i64, i64) = (xmin.parse().unwrap(), xmax.parse().unwrap());
    let (ymin, ymax): (i64, i64) = (ymin.parse().unwrap(), ymax.parse().unwrap());

    let xvals = day17_xvals(xmin, xmax);
    let mut max = 0;
    for xvel in xvals {
        for yvel in 0..100 {
            if let Some(xposmax) = day17_hits(xvel, yvel, xmin, ymin, xmax, ymax) {
                if xposmax > max {
                    max = xposmax;
                }
            }
        }
    }
    Ok(max as usize)
}

#[inline(never)]
pub fn day17_puzzle2() -> Result<usize, std::io::Error> {
    let data = std::fs::read_to_string("inputs/input-17")?;
    let (xs, ys) = data.split_once(", ").unwrap();
    let (_, xs) = xs.split_once("=").unwrap();
    let (_, ys) = ys.split_once("=").unwrap();
    let (xmin, xmax) = xs.split_once("..").unwrap();
    let (ymin, ymax) = ys.split_once("..").unwrap();
    let (xmin, xmax): (i64, i64) = (xmin.parse().unwrap(), xmax.parse().unwrap());
    let (ymin, ymax): (i64, i64) = (ymin.parse().unwrap(), ymax.parse().unwrap());

    let xvals = day17_xvals(xmin, xmax);
    let mut count = 0;
    for xvel in xvals {
        for yvel in ymin - 1..100 {
            if day17_hits(xvel, yvel, xmin, ymin, xmax, ymax).is_some() {
                count += 1;
            }
        }
    }
    Ok(count as usize)
}

#[derive(Clone, Debug)]
enum Sn {
    Value(i64),
    Pair(Box<Sn>, Box<Sn>),
}

fn day18_parse(s: &str) -> (Box<Sn>, &str) {
    match s.chars().next().unwrap() {
        '[' => {
            let (a, tail) = day18_parse(&s[1..]);
            assert_eq!(tail.chars().next().unwrap(), ',');
            let (b, tail) = day18_parse(&tail[1..]);
            assert_eq!(tail.chars().next().unwrap(), ']');
            (Box::new(Sn::Pair(a, b)), &tail[1..])
        }
        v @ '0'..='9' => (Box::new(Sn::Value(v.to_digit(10).unwrap().into())), &s[1..]),
        _ => panic!("invalid input"),
    }
}

fn day18_mag(sn: &Box<Sn>) -> i64 {
    match &**sn {
        Sn::Value(v) => *v,
        Sn::Pair(a, b) => (day18_mag(&a) * 3) + (day18_mag(&b) * 2),
    }
}

fn day18_addleft(sn: Box<Sn>, x: i64) -> Box<Sn> {
    match *sn {
        Sn::Value(v) => Box::new(Sn::Value(v + x)),
        Sn::Pair(a, b) => Box::new(Sn::Pair(a, day18_addleft(b, x))),
    }
}

fn day18_addright(sn: Box<Sn>, x: i64) -> Box<Sn> {
    match *sn {
        Sn::Value(v) => Box::new(Sn::Value(v + x)),
        Sn::Pair(a, b) => Box::new(Sn::Pair(day18_addright(a, x), b)),
    }
}

fn day18_getval(sn: &Box<Sn>) -> i64 {
    if let Sn::Value(v) = **sn {
        v
    } else {
        panic!("invalid input")
    }
}

fn day18_explode(sn: Box<Sn>, level: i64) -> (Box<Sn>, bool, Option<i64>, Option<i64>) {
    match *sn {
        Sn::Value(v) => (Box::new(Sn::Value(v)), false, None, None),
        Sn::Pair(a, b) => {
            if level >= 4 {
                let a = day18_getval(&a);
                let b = day18_getval(&b);
                (Box::new(Sn::Value(0)), true, Some(a), Some(b))
            } else {
                let (a, e, lo, ro) = day18_explode(a, level + 1);
                if e {
                    let b = if let Some(r) = ro {
                        day18_addright(b, r)
                    } else {
                        b
                    };
                    (Box::new(Sn::Pair(a, b)), e, lo, None)
                } else {
                    let (b, e, lo, ro) = day18_explode(b, level + 1);
                    if e {
                        let a = if let Some(l) = lo {
                            day18_addleft(a, l)
                        } else {
                            a
                        };
                        (Box::new(Sn::Pair(a, b)), e, None, ro)
                    } else {
                        (Box::new(Sn::Pair(a, b)), false, None, None)
                    }
                }
            }
        }
    }
}

fn day18_split(sn: Box<Sn>) -> (Box<Sn>, bool) {
    match *sn {
        Sn::Value(v) => {
            if v > 9 {
                (
                    Box::new(Sn::Pair(
                        Box::new(Sn::Value(v / 2)),
                        Box::new(Sn::Value((v + 1) / 2)),
                    )),
                    true,
                )
            } else {
                (Box::new(Sn::Value(v)), false)
            }
        }
        Sn::Pair(a, b) => {
            let (a, s) = day18_split(a);
            if s {
                (Box::new(Sn::Pair(a, b)), s)
            } else {
                let (b, s) = day18_split(b);
                (Box::new(Sn::Pair(a, b)), s)
            }
        }
    }
}

fn day18_reduce(sn: Box<Sn>) -> Box<Sn> {
    let mut ret = sn;
    loop {
        let (n, e, _, _) = day18_explode(ret, 0);
        ret = n;
        if e {
            continue;
        }
        let (n, s) = day18_split(ret);
        ret = n;
        if s {
            continue;
        }
        break;
    }
    ret
}

fn day18_add(a: Box<Sn>, b: Box<Sn>) -> Box<Sn> {
    day18_reduce(Box::new(Sn::Pair(a, b)))
}

#[inline(never)]
pub fn day18_puzzle1() -> Result<usize, std::io::Error> {
    let data = std::fs::read_to_string("inputs/input-18")?
        .lines()
        .map(|l| day18_parse(l).0)
        .collect::<Vec<Box<Sn>>>();
    let sum = data.into_iter().reduce(day18_add).unwrap();
    let ret = day18_mag(&sum);
    Ok(ret as usize)
}

#[inline(never)]
pub fn day18_puzzle2() -> Result<usize, std::io::Error> {
    let data = std::fs::read_to_string("inputs/input-18")?
        .lines()
        .map(|l| day18_parse(l).0)
        .collect::<Vec<Box<Sn>>>();
    let ret = data
        .iter()
        .enumerate()
        .map(|(i, a)| {
            data[i..]
                .iter()
                .map(|b| day18_mag(&day18_add(a.clone(), b.clone())))
                .max()
                .unwrap()
        })
        .max()
        .unwrap();
    Ok(ret as usize)
}

fn day19_read_data(filename: &str) -> Result<Vec<Vec<(i64, i64, i64)>>, std::io::Error> {
    let data = std::fs::read_to_string(filename)?
        .split("\n\n")
        .enumerate()
        .map(|(_i, scanner)| {
            let (_, scanner) = scanner.split_once("\n").unwrap();
            scanner
                .lines()
                .map(|l| {
                    let mut l = l.split(",").map(|c| c.parse::<i64>().unwrap());
                    (l.next().unwrap(), l.next().unwrap(), l.next().unwrap())
                })
                .collect()
        })
        .collect();
    Ok(data)
}

fn day19_per_coord(p: i64, (x, y, z): (i64, i64, i64)) -> (i64, i64, i64) {
    match p {
        0 => (x, y, z),
        1 => (x, z, -y),
        2 => (x, -y, -z),
        3 => (x, -z, y),
        4 => (y, -x, z),
        5 => (y, x, z),
        6 => (y, x, -z),
        7 => (y, -z, -x),
        8 => (-x, -y, z),
        9 => (-x, -z, -y),
        10 => (-x, y, -z),
        11 => (-x, z, y),
        12 => (-y, x, z),
        13 => (-y, -z, x),
        14 => (-y, -x, -z),
        15 => (-y, z, -x),
        16 => (z, y, -x),
        17 => (z, x, y),
        18 => (z, -y, x),
        19 => (z, -x, -y),
        20 => (-z, -y, -x),
        21 => (-z, -x, y),
        22 => (-z, y, x),
        23 => (-z, x, -y),
        _ => panic!("invalid input"),
    }
}

fn day19_per(p: i64, v: &mut Vec<(i64, i64, i64)>) {
    v.iter_mut().for_each(|x| *x = day19_per_coord(p, *x));
}

fn day19_count(a: &Vec<(i64, i64, i64)>, b: &Vec<(i64, i64, i64)>) -> bool {
    let mut count = 0;
    for (i, bv) in b.iter().enumerate() {
        if a.contains(bv) {
            count += 1;
            if count >= 12 {
                return true;
            }
        }
        if b.len() - i + 1 + count < 12 {
            return false;
        }
    }
    false
}

fn day19_add((x1, y1, z1): (i64, i64, i64), (x2, y2, z2): (i64, i64, i64)) -> (i64, i64, i64) {
    (x1 + x2, y1 + y2, z1 + z2)
}

fn day19_sub((x1, y1, z1): (i64, i64, i64), (x2, y2, z2): (i64, i64, i64)) -> (i64, i64, i64) {
    (x1 - x2, y1 - y2, z1 - z2)
}

fn day19_offset((x1, y1, z1): (i64, i64, i64), a: &mut Vec<(i64, i64, i64)>) {
    a.iter_mut()
        .for_each(|v| *v = (v.0 + x1, v.1 + y1, v.2 + z1));
}

fn day19_matches_offset(
    a: &Vec<(i64, i64, i64)>,
    b: &Vec<(i64, i64, i64)>,
) -> Option<(i64, i64, i64)> {
    for v1 in a[..14].iter() {
        for v2 in b[..14].iter() {
            let offset = day19_sub(*v1, *v2);
            let mut offsetted = b.clone();
            day19_offset(offset, &mut offsetted);
            if day19_count(a, &offsetted) {
                return Some(offset);
            }
        }
    }
    None
}

fn day19_matches_per(
    a: &Vec<(i64, i64, i64)>,
    b: &Vec<(i64, i64, i64)>,
) -> Option<(i64, (i64, i64, i64))> {
    for p in 0..24 {
        let mut pv = b.clone();
        day19_per(p, &mut pv);
        if let Some(offset) = day19_matches_offset(a, &pv) {
            return Some((p, offset));
        }
    }
    None
}

fn day19_solve(
    beacons: &mut Vec<(i64, i64, i64)>,
    added: &mut Vec<usize>,
    offsets: &mut Vec<(i64, i64, i64)>,
    matching: &Vec<(usize, usize, i64, (i64, i64, i64))>,
    data: &Vec<Vec<(i64, i64, i64)>>,
    cur: usize,
    trans: &Vec<(i64, (i64, i64, i64))>,
) {
    for &(i, j, p, offset) in matching {
        if i == cur && !added.contains(&j) {
            let mut trans = trans.clone();
            trans.push((p, offset));
            let mut permuted = data[j].clone();
            let mut totaloff = (0, 0, 0);
            for &(pastp, pastoff) in trans.iter().rev() {
                day19_per(pastp, &mut permuted);
                day19_offset(pastoff, &mut permuted);
                totaloff = day19_add(pastoff, day19_per_coord(pastp, totaloff));
            }
            beacons.extend(permuted);
            added.push(j);
            offsets.push(totaloff);
            day19_solve(beacons, added, offsets, matching, data, j, &trans);
        }
    }
}

#[cached(key = "bool", convert = "{ false }")] // hack to avoid calculating matching set twice
fn day19_build_matching(
    data: &Vec<Vec<(i64, i64, i64)>>,
) -> Vec<(usize, usize, i64, (i64, i64, i64))> {
    let mut matching = Vec::new();
    for (i, a) in data.iter().enumerate() {
        for (j, b) in data.iter().enumerate() {
            if i == j {
                continue;
            }
            if let Some((p, offset)) = day19_matches_per(a, b) {
                matching.push((i, j, p, offset));
                //println!("i {} j {} p {} offset {:?}", i, j, p, offset);
            }
        }
    }
    matching
}

#[inline(never)]
pub fn day19_puzzle1() -> Result<usize, std::io::Error> {
    let data = day19_read_data("inputs/input-19")?;
    let matching = day19_build_matching(&data);
    let mut beacons = Vec::new();
    let mut added = Vec::new();
    let mut offsets = Vec::new();
    beacons.extend(&data[0]);
    added.push(0);
    day19_solve(
        &mut beacons,
        &mut added,
        &mut offsets,
        &matching,
        &data,
        0,
        &Vec::new(),
    );
    beacons.sort();
    beacons.dedup();
    Ok(beacons.len() as usize)
}

#[inline(never)]
pub fn day19_puzzle2() -> Result<usize, std::io::Error> {
    let data = day19_read_data("inputs/input-19")?;
    let matching = day19_build_matching(&data);
    let mut beacons = Vec::new();
    let mut added = Vec::new();
    let mut offsets = Vec::new();
    beacons.extend(&data[0]);
    added.push(0);
    day19_solve(
        &mut beacons,
        &mut added,
        &mut offsets,
        &matching,
        &data,
        0,
        &Vec::new(),
    );
    let mut maxdis = 0;
    for &a in offsets.iter() {
        for &b in offsets.iter() {
            let off = day19_sub(b, a);
            let dis = off.0.abs() + off.1.abs() + off.2.abs();
            if dis > maxdis {
                maxdis = dis;
            }
        }
    }
    Ok(maxdis as usize)
}

fn day20_ch_to_bool(ch: char) -> bool {
    match ch {
        '#' => true,
        '.' => false,
        _ => panic!("invalid input"),
    }
}

fn day20_getint<const L: usize>(image: &[[bool; L]; L], x: usize, y: usize) -> usize {
    let mut val = 0usize;
    for row in y - 1..=y + 1 {
        for col in x - 1..=x + 1 {
            val = val << 1 | image[row][col] as usize;
        }
    }
    val
}

fn day20_enhance<const L: usize>(
    image: &mut [[bool; L]; L],
    future: &mut [[bool; L]; L],
    algo: &Vec<bool>,
) {
    let xlen = image[0].len();
    let ylen = image.len();
    for x in 0..xlen {
        future[0][x] = !image[0][x];
        future[ylen - 1][x] = !image[ylen - 1][x];
    }
    for y in 1..ylen - 1 {
        future[y][0] = !image[y][0];
        for x in 1..xlen - 1 {
            future[y][x] = algo[day20_getint(image, x, y)];
        }
        future[y][xlen - 1] = !image[y][xlen - 1];
    }
}

#[inline(never)]
pub fn day20_puzzle1() -> Result<usize, std::io::Error> {
    const N: usize = 2;
    const SIZ: usize = 100 + 2 + (N * 2);
    let data = std::fs::read_to_string("inputs/input-20")?;
    let (algostr, imagestr) = data.split_once("\n\n").unwrap();
    let algo: Vec<bool> = algostr.chars().map(day20_ch_to_bool).collect();
    let mut image = [[false; SIZ]; SIZ];
    let mut future = [[false; SIZ]; SIZ];
    imagestr.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| {
            image[row + ((SIZ - 100) / 2)][col + ((SIZ - 100) / 2)] = day20_ch_to_bool(c);
        })
    });
    for _ in 0..N {
        day20_enhance(&mut image, &mut future, &algo);
        std::mem::swap(&mut image, &mut future);
    }
    let ret: i64 = image
        .iter()
        .map(|r| r.iter().map(|&c| c as i64).sum::<i64>())
        .sum::<i64>();
    Ok(ret as usize)
}

#[inline(never)]
pub fn day20_puzzle2() -> Result<usize, std::io::Error> {
    const N: usize = 50;
    const SIZ: usize = 100 + 2 + (N * 2);
    let data = std::fs::read_to_string("inputs/input-20")?;
    let (algostr, imagestr) = data.split_once("\n\n").unwrap();
    let algo: Vec<bool> = algostr.chars().map(day20_ch_to_bool).collect();
    let mut image = [[false; SIZ]; SIZ];
    let mut future = [[false; SIZ]; SIZ];
    imagestr.lines().enumerate().for_each(|(row, line)| {
        line.chars().enumerate().for_each(|(col, c)| {
            image[row + ((SIZ - 100) / 2)][col + ((SIZ - 100) / 2)] = day20_ch_to_bool(c);
        })
    });
    for _ in 0..N {
        day20_enhance(&mut image, &mut future, &algo);
        std::mem::swap(&mut image, &mut future);
    }
    let ret: i64 = image
        .iter()
        .map(|r| r.iter().map(|&c| c as i64).sum::<i64>())
        .sum::<i64>();
    Ok(ret as usize)
}

fn day21_read_data(filename: &str) -> Result<(u8, u8), std::io::Error> {
    let data = std::fs::read_to_string(filename)?;
    let mut lines = data.lines();
    let apos = lines.next().unwrap()[28..].parse::<u8>().unwrap();
    let bpos = lines.next().unwrap()[28..].parse::<u8>().unwrap();
    Ok((apos, bpos))
}

#[inline(never)]
pub fn day21_puzzle1() -> Result<usize, std::io::Error> {
    let (p1pos, p2pos) = day21_read_data("inputs/input-21")?;
    let mut die = 1;
    let mut rolls: i64 = 0;
    let mut state: (i64, i64, i64, i64) = (p1pos.into(), 0, p2pos.into(), 0);
    loop {
        rolls += 3;
        let (apos, ascore, bpos, bscore) = state;
        let mut roll = 0;
        for _ in 0..3 {
            roll += die;
            die = if die >= 100 { 1 } else { die + 1 };
        }
        let cpos = (apos + roll) % 10;
        let cscore = ascore + cpos;
        state = (bpos, bscore, cpos, cscore);
        if cscore >= 1000 {
            break;
        }
    }
    let (_, lscore, _, _) = state;
    let ret = lscore * rolls;
    Ok(ret as usize)
}

#[inline(never)]
pub fn day21_puzzle2() -> Result<usize, std::io::Error> {
    let (apos, bpos) = day21_read_data("inputs/input-21")?;
    let mut universes: BTreeMap<(u8, u8, u8, u8, bool), i64> = BTreeMap::new();
    universes.insert((apos, 0, bpos, 0, true), 1);
    let mut p1wins: i64 = 0;
    let mut p2wins: i64 = 0;
    let rolls = [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
    while !universes.is_empty() {
        let (&key, &count) = universes.iter().next().unwrap();
        let (apos, ascore, bpos, bscore, p1up) = key;
        universes.remove(&key);
        for (v, m) in rolls {
            let cpos = if (apos + v) > 10 {
                apos + v - 10
            } else {
                apos + v
            };
            let cscore = ascore + cpos;
            if cscore >= 21 {
                if p1up {
                    p1wins += count * m;
                } else {
                    p2wins += count * m;
                }
            } else {
                let key = (bpos, bscore, cpos, cscore, !p1up);
                *universes.entry(key).or_insert(0) += count * m;
            }
        }
    }
    let ret = max(p1wins, p2wins);
    Ok(ret as usize)
}

type Day22Point = (i64, i64, i64);
type Day22Step = (bool, Day22Point, Day22Point);

fn day22_read_data(filename: &str) -> Result<Vec<Day22Step>, std::io::Error> {
    let data = std::fs::read_to_string(filename)?
        .lines()
        .map(|l| {
            let (onoff, rest) = l.split_once(" ").unwrap();
            let (xrange, rest) = rest.split_once(",").unwrap();
            let (yrange, zrange) = rest.split_once(",").unwrap();
            let (xmin, xmax) = xrange.split_once("=").unwrap().1.split_once("..").unwrap();
            let (ymin, ymax) = yrange.split_once("=").unwrap().1.split_once("..").unwrap();
            let (zmin, zmax) = zrange.split_once("=").unwrap().1.split_once("..").unwrap();
            (
                onoff == "on",
                (
                    xmin.parse::<i64>().unwrap(),
                    ymin.parse::<i64>().unwrap(),
                    zmin.parse::<i64>().unwrap(),
                ),
                (
                    xmax.parse::<i64>().unwrap(),
                    ymax.parse::<i64>().unwrap(),
                    zmax.parse::<i64>().unwrap(),
                ),
            )
        })
        .collect();
    Ok(data)
}

#[inline(never)]
pub fn day22_puzzle1() -> Result<usize, std::io::Error> {
    let data = day22_read_data("inputs/input-22")?;
    let mut core = [[[false; 101]; 101]; 101];
    for &(onoff, (xmin, ymin, zmin), (xmax, ymax, zmax)) in data.iter() {
        for z in max(zmin, -50)..=min(zmax, 50) {
            for y in max(ymin, -50)..=min(ymax, 50) {
                for x in max(xmin, -50)..=min(xmax, 50) {
                    core[(z + 50) as usize][(y + 50) as usize][(x + 50) as usize] = onoff;
                }
            }
        }
    }
    let count = core
        .iter()
        .map(|z| {
            z.iter()
                .map(|y| y.iter().map(|&x| x as i64).sum::<i64>())
                .sum::<i64>()
        })
        .sum::<i64>();
    Ok(count as usize)
}

fn day22_split(a: Day22Step, b: Day22Step) -> Vec<Day22Step> {
    let (t, (axmin, aymin, azmin), (axmax, aymax, azmax)) = a;
    let (_, (bxmin, bymin, bzmin), (bxmax, bymax, bzmax)) = b;

    if !((bxmax >= axmin)
        && (bymax >= aymin)
        && (bzmax >= azmin)
        && (bxmin <= axmax)
        && (bymin <= aymax)
        && (bzmin <= azmax))
    {
        return vec![a];
    }

    let mut c = Vec::new();
    let (ozmin, ozmax) = (bzmax + 1, bzmin - 1);
    let (izmin, izmax) = (max(bzmin, azmin), min(bzmax, azmax));
    let (oymin, oymax) = (bymax + 1, bymin - 1);
    let (iymin, iymax) = (max(bymin, aymin), min(bymax, aymax));
    let (oxmin, oxmax) = (bxmax + 1, bxmin - 1);
    (azmin < bzmin).then(|| c.push((t, (axmin, aymin, azmin), (axmax, aymax, ozmax))));
    (azmax > bzmax).then(|| c.push((t, (axmin, aymin, ozmin), (axmax, aymax, azmax))));
    (aymin < bymin).then(|| c.push((t, (axmin, aymin, izmin), (axmax, oymax, izmax))));
    (aymax > bymax).then(|| c.push((t, (axmin, oymin, izmin), (axmax, aymax, izmax))));
    (axmin < bxmin).then(|| c.push((t, (axmin, iymin, izmin), (oxmax, iymax, izmax))));
    (axmax > bxmax).then(|| c.push((t, (oxmin, iymin, izmin), (axmax, iymax, izmax))));
    c
}

fn day22_count(a: Day22Step) -> i64 {
    let (t, (xmin, ymin, zmin), (xmax, ymax, zmax)) = a;
    (t as i64) * (1 + xmax - xmin) * (1 + ymax - ymin) * (1 + zmax - zmin)
}

#[inline(never)]
pub fn day22_puzzle2() -> Result<usize, std::io::Error> {
    let data = day22_read_data("inputs/input-22")?;
    let mut cubes: Vec<Day22Step> = Vec::new();
    for &d in &data {
        cubes = cubes.iter().flat_map(|&a| day22_split(a, d)).collect();
        if d.0 {
            cubes.push(d);
        }
    }
    let mut count = 0;
    for &c in &cubes {
        count += day22_count(c);
    }
    Ok(count as usize)
}

fn day23_a_to_n(ch: char) -> u8 {
    match ch {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        'D' => 4,
        _ => panic!("invalid input"),
    }
}

fn day23_cost(a: u8) -> i64 {
    match a {
        1 => 1,
        2 => 10,
        3 => 100,
        4 => 1000,
        _ => panic!("invalid input"),
    }
}

fn day23_read_data(filename: &str) -> Result<[u8; 15], std::io::Error> {
    let data = std::fs::read_to_string(filename)?;
    let lines = data.lines().collect::<Vec<&str>>();
    let mut state = [0u8; 15];
    state[0] = day23_a_to_n(lines[3].chars().nth(3).unwrap());
    state[1] = day23_a_to_n(lines[3].chars().nth(5).unwrap());
    state[2] = day23_a_to_n(lines[3].chars().nth(7).unwrap());
    state[3] = day23_a_to_n(lines[3].chars().nth(9).unwrap());
    state[4] = day23_a_to_n(lines[2].chars().nth(3).unwrap());
    state[5] = day23_a_to_n(lines[2].chars().nth(5).unwrap());
    state[6] = day23_a_to_n(lines[2].chars().nth(7).unwrap());
    state[7] = day23_a_to_n(lines[2].chars().nth(9).unwrap());
    Ok(state)
}

fn day23_move<const N: usize>(state: &[u8; N], from: usize, to: usize, n: i64) -> ([u8; N], i64) {
    let mut ret = state.clone();
    ret[to] = ret[from];
    ret[from] = 0;
    (ret, day23_cost(ret[to]) * n)
}

fn day23_moves<const N: usize>(
    paths: &Vec<(u8, Vec<usize>, Vec<usize>, i64)>,
    state: &[u8; N],
) -> Vec<([u8; N], i64)> {
    let mut ret = Vec::new();
    for (lane, sta, path, n) in paths {
        let a = *path.first().unwrap();
        let b = *path.last().unwrap();
        if state[a] != 0 && !path.iter().skip(1).any(|&x| state[x] != 0) {
            ret.push(day23_move(state, a, b, *n));
        }
        if state[b] != 0 && !path.iter().rev().skip(1).any(|&x| state[x] != 0) {
            if *lane == state[b] && !sta.iter().any(|&x| state[x] != *lane) {
                ret.push(day23_move(state, b, a, *n));
            }
        }
    }
    ret
}

fn day23_done(state: &[u8; 15]) -> bool {
    *state == [1, 2, 3, 4, 1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0]
}

#[inline(never)]
pub fn day23_puzzle1() -> Result<usize, std::io::Error> {
    let data = day23_read_data("inputs/input-23")?;
    let paths = vec![
        (1, vec![], vec![0, 4, 9, 8], 4),
        (1, vec![], vec![0, 4, 9], 3),
        (1, vec![], vec![0, 4, 10], 3),
        (1, vec![], vec![0, 4, 10, 11], 5),
        (1, vec![], vec![0, 4, 10, 11, 12], 7),
        (1, vec![], vec![0, 4, 10, 11, 12, 13], 9),
        (1, vec![], vec![0, 4, 10, 11, 12, 13, 14], 10),
        (2, vec![], vec![1, 5, 10, 9, 8], 6),
        (2, vec![], vec![1, 5, 10, 9], 5),
        (2, vec![], vec![1, 5, 10], 3),
        (2, vec![], vec![1, 5, 11], 3),
        (2, vec![], vec![1, 5, 11, 12], 5),
        (2, vec![], vec![1, 5, 11, 12, 13], 7),
        (2, vec![], vec![1, 5, 11, 12, 13, 14], 8),
        (3, vec![], vec![2, 6, 11, 10, 9, 8], 8),
        (3, vec![], vec![2, 6, 11, 10, 9], 7),
        (3, vec![], vec![2, 6, 11, 10], 5),
        (3, vec![], vec![2, 6, 11], 3),
        (3, vec![], vec![2, 6, 12], 3),
        (3, vec![], vec![2, 6, 12, 13], 5),
        (3, vec![], vec![2, 6, 12, 13, 14], 6),
        (4, vec![], vec![3, 7, 12, 11, 10, 9, 8], 10),
        (4, vec![], vec![3, 7, 12, 11, 10, 9], 9),
        (4, vec![], vec![3, 7, 12, 11, 10], 7),
        (4, vec![], vec![3, 7, 12, 11], 5),
        (4, vec![], vec![3, 7, 12], 3),
        (4, vec![], vec![3, 7, 13], 3),
        (4, vec![], vec![3, 7, 13, 14], 4),
        (1, vec![0], vec![4, 9, 8], 3),
        (1, vec![0], vec![4, 9], 2),
        (1, vec![0], vec![4, 10], 2),
        (1, vec![0], vec![4, 10, 11], 4),
        (1, vec![0], vec![4, 10, 11, 12], 6),
        (1, vec![0], vec![4, 10, 11, 12, 13], 8),
        (1, vec![0], vec![4, 10, 11, 12, 13, 14], 9),
        (2, vec![1], vec![5, 10, 9, 8], 5),
        (2, vec![1], vec![5, 10, 9], 4),
        (2, vec![1], vec![5, 10], 2),
        (2, vec![1], vec![5, 11], 2),
        (2, vec![1], vec![5, 11, 12], 4),
        (2, vec![1], vec![5, 11, 12, 13], 6),
        (2, vec![1], vec![5, 11, 12, 13, 14], 7),
        (3, vec![2], vec![6, 11, 10, 9, 8], 7),
        (3, vec![2], vec![6, 11, 10, 9], 6),
        (3, vec![2], vec![6, 11, 10], 4),
        (3, vec![2], vec![6, 11], 2),
        (3, vec![2], vec![6, 12], 2),
        (3, vec![2], vec![6, 12, 13], 4),
        (3, vec![2], vec![6, 12, 13, 14], 5),
        (4, vec![3], vec![7, 12, 11, 10, 9, 8], 9),
        (4, vec![3], vec![7, 12, 11, 10, 9], 8),
        (4, vec![3], vec![7, 12, 11, 10], 6),
        (4, vec![3], vec![7, 12, 11], 4),
        (4, vec![3], vec![7, 12], 2),
        (4, vec![3], vec![7, 13], 2),
        (4, vec![3], vec![7, 13, 14], 3),
    ];
    let (_path, cost) = astar(&data, |m| day23_moves(&paths, m), |_| 1, day23_done).unwrap();
    Ok(cost as usize)
}

fn day23_done2(state: &[u8; 23]) -> bool {
    *state
        == [
            1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 1, 2, 3, 4, 0, 0, 0, 0, 0, 0, 0,
        ]
}

fn day23_extend(data: &[u8; 15]) -> [u8; 23] {
    let mut ret = [0u8; 23];
    ret[0..4].copy_from_slice(&data[0..4]);
    ret[4..8].copy_from_slice(&[4, 2, 1, 3]);
    ret[8..12].copy_from_slice(&[4, 3, 2, 1]);
    ret[12..16].copy_from_slice(&data[4..8]);
    ret
}

#[inline(never)]
pub fn day23_puzzle2() -> Result<usize, std::io::Error> {
    let data = day23_read_data("inputs/input-23")?;
    let data = day23_extend(&data);
    let paths = vec![
        (1, vec![0, 4, 8], vec![12, 17, 16], 3),
        (1, vec![0, 4, 8], vec![12, 17], 2),
        (1, vec![0, 4, 8], vec![12, 18], 2),
        (1, vec![0, 4, 8], vec![12, 18, 19], 4),
        (1, vec![0, 4, 8], vec![12, 18, 19, 20], 6),
        (1, vec![0, 4, 8], vec![12, 18, 19, 20, 21], 8),
        (1, vec![0, 4, 8], vec![12, 18, 19, 20, 21, 22], 9),
        (2, vec![1, 5, 9], vec![13, 18, 17, 16], 5),
        (2, vec![1, 5, 9], vec![13, 18, 17], 4),
        (2, vec![1, 5, 9], vec![13, 18], 2),
        (2, vec![1, 5, 9], vec![13, 19], 2),
        (2, vec![1, 5, 9], vec![13, 19, 20], 4),
        (2, vec![1, 5, 9], vec![13, 19, 20, 21], 6),
        (2, vec![1, 5, 9], vec![13, 19, 20, 21, 22], 7),
        (3, vec![2, 6, 10], vec![14, 19, 18, 17, 16], 7),
        (3, vec![2, 6, 10], vec![14, 19, 18, 17], 6),
        (3, vec![2, 6, 10], vec![14, 19, 18], 4),
        (3, vec![2, 6, 10], vec![14, 19], 2),
        (3, vec![2, 6, 10], vec![14, 20], 2),
        (3, vec![2, 6, 10], vec![14, 20, 21], 4),
        (3, vec![2, 6, 10], vec![14, 20, 21, 22], 5),
        (4, vec![3, 7, 11], vec![15, 20, 19, 18, 17, 16], 9),
        (4, vec![3, 7, 11], vec![15, 20, 19, 18, 17], 8),
        (4, vec![3, 7, 11], vec![15, 20, 19, 18], 6),
        (4, vec![3, 7, 11], vec![15, 20, 19], 4),
        (4, vec![3, 7, 11], vec![15, 20], 2),
        (4, vec![3, 7, 11], vec![15, 21], 2),
        (4, vec![3, 7, 11], vec![15, 21, 22], 3),
        (1, vec![0, 4], vec![8, 12, 17, 16], 4),
        (1, vec![0, 4], vec![8, 12, 17], 3),
        (1, vec![0, 4], vec![8, 12, 18], 3),
        (1, vec![0, 4], vec![8, 12, 18, 19], 5),
        (1, vec![0, 4], vec![8, 12, 18, 19, 20], 7),
        (1, vec![0, 4], vec![8, 12, 18, 19, 20, 21], 9),
        (1, vec![0, 4], vec![8, 12, 18, 19, 20, 21, 22], 10),
        (2, vec![1, 5], vec![9, 13, 18, 17, 16], 6),
        (2, vec![1, 5], vec![9, 13, 18, 17], 5),
        (2, vec![1, 5], vec![9, 13, 18], 3),
        (2, vec![1, 5], vec![9, 13, 19], 3),
        (2, vec![1, 5], vec![9, 13, 19, 20], 5),
        (2, vec![1, 5], vec![9, 13, 19, 20, 21], 7),
        (2, vec![1, 5], vec![9, 13, 19, 20, 21, 22], 8),
        (3, vec![2, 6], vec![10, 14, 19, 18, 17, 16], 8),
        (3, vec![2, 6], vec![10, 14, 19, 18, 17], 7),
        (3, vec![2, 6], vec![10, 14, 19, 18], 5),
        (3, vec![2, 6], vec![10, 14, 19], 3),
        (3, vec![2, 6], vec![10, 14, 20], 3),
        (3, vec![2, 6], vec![10, 14, 20, 21], 5),
        (3, vec![2, 6], vec![10, 14, 20, 21, 22], 6),
        (4, vec![3, 7], vec![11, 15, 20, 19, 18, 17, 16], 10),
        (4, vec![3, 7], vec![11, 15, 20, 19, 18, 17], 9),
        (4, vec![3, 7], vec![11, 15, 20, 19, 18], 7),
        (4, vec![3, 7], vec![11, 15, 20, 19], 5),
        (4, vec![3, 7], vec![11, 15, 20], 3),
        (4, vec![3, 7], vec![11, 15, 21], 3),
        (4, vec![3, 7], vec![11, 15, 21, 22], 5),
        (1, vec![0], vec![4, 8, 12, 17, 16], 5),
        (1, vec![0], vec![4, 8, 12, 17], 4),
        (1, vec![0], vec![4, 8, 12, 18], 4),
        (1, vec![0], vec![4, 8, 12, 18, 19], 6),
        (1, vec![0], vec![4, 8, 12, 18, 19, 20], 8),
        (1, vec![0], vec![4, 8, 12, 18, 19, 20, 21], 10),
        (1, vec![0], vec![4, 8, 12, 18, 19, 20, 21, 22], 11),
        (2, vec![1], vec![5, 9, 13, 18, 17, 16], 8),
        (2, vec![1], vec![5, 9, 13, 18, 17], 6),
        (2, vec![1], vec![5, 9, 13, 18], 4),
        (2, vec![1], vec![5, 9, 13, 19], 4),
        (2, vec![1], vec![5, 9, 13, 19, 20], 6),
        (2, vec![1], vec![5, 9, 13, 19, 20, 21], 8),
        (2, vec![1], vec![5, 9, 13, 19, 20, 21, 22], 9),
        (3, vec![2], vec![6, 10, 14, 19, 18, 17, 16], 9),
        (3, vec![2], vec![6, 10, 14, 19, 18, 17], 8),
        (3, vec![2], vec![6, 10, 14, 19, 18], 6),
        (3, vec![2], vec![6, 10, 14, 19], 4),
        (3, vec![2], vec![6, 10, 14, 20], 4),
        (3, vec![2], vec![6, 10, 14, 20, 21], 6),
        (3, vec![2], vec![6, 10, 14, 20, 21, 22], 7),
        (4, vec![3], vec![7, 11, 15, 20, 19, 18, 17, 16], 11),
        (4, vec![3], vec![7, 11, 15, 20, 19, 18, 17], 10),
        (4, vec![3], vec![7, 11, 15, 20, 19, 18], 8),
        (4, vec![3], vec![7, 11, 15, 20, 19], 6),
        (4, vec![3], vec![7, 11, 15, 20], 4),
        (4, vec![3], vec![7, 11, 15, 21], 4),
        (4, vec![3], vec![7, 11, 15, 21, 22], 5),
        (1, vec![], vec![0, 4, 8, 12, 17, 16], 6),
        (1, vec![], vec![0, 4, 8, 12, 17], 5),
        (1, vec![], vec![0, 4, 8, 12, 18], 5),
        (1, vec![], vec![0, 4, 8, 12, 18, 19], 7),
        (1, vec![], vec![0, 4, 8, 12, 18, 19, 20], 9),
        (1, vec![], vec![0, 4, 8, 12, 18, 19, 20, 21], 11),
        (1, vec![], vec![0, 4, 8, 12, 18, 19, 20, 21, 22], 12),
        (2, vec![], vec![1, 5, 9, 13, 18, 17, 16], 8),
        (2, vec![], vec![1, 5, 9, 13, 18, 17], 7),
        (2, vec![], vec![1, 5, 9, 13, 18], 5),
        (2, vec![], vec![1, 5, 9, 13, 19], 5),
        (2, vec![], vec![1, 5, 9, 13, 19, 20], 7),
        (2, vec![], vec![1, 5, 9, 13, 19, 20, 21], 9),
        (2, vec![], vec![1, 5, 9, 13, 19, 20, 21, 22], 11),
        (3, vec![], vec![2, 6, 10, 14, 19, 18, 17, 16], 10),
        (3, vec![], vec![2, 6, 10, 14, 19, 18, 17], 9),
        (3, vec![], vec![2, 6, 10, 14, 19, 18], 7),
        (3, vec![], vec![2, 6, 10, 14, 19], 5),
        (3, vec![], vec![2, 6, 10, 14, 20], 5),
        (3, vec![], vec![2, 6, 10, 14, 20, 21], 7),
        (3, vec![], vec![2, 6, 10, 14, 20, 21, 22], 8),
        (4, vec![], vec![3, 7, 11, 15, 20, 19, 18, 17, 16], 12),
        (4, vec![], vec![3, 7, 11, 15, 20, 19, 18, 17], 11),
        (4, vec![], vec![3, 7, 11, 15, 20, 19, 18], 9),
        (4, vec![], vec![3, 7, 11, 15, 20, 19], 7),
        (4, vec![], vec![3, 7, 11, 15, 20], 5),
        (4, vec![], vec![3, 7, 11, 15, 21], 5),
        (4, vec![], vec![3, 7, 11, 15, 21, 22], 7),
    ];
    let (_path, cost) = astar(&data, |m| day23_moves(&paths, m), |_| 1, day23_done2).unwrap();
    Ok(cost as usize)
}

#[derive(Copy, Clone, Debug)]
enum Day24RegVal {
    W,
    X,
    Y,
    Z,
    Value(i64),
}

#[derive(Copy, Clone, Debug)]
enum Day24Inst {
    Inp(Day24RegVal),
    Add(Day24RegVal, Day24RegVal),
    Mul(Day24RegVal, Day24RegVal),
    Div(Day24RegVal, Day24RegVal),
    Mod(Day24RegVal, Day24RegVal),
    Eql(Day24RegVal, Day24RegVal),
}

fn day24_parse_regval(r: &str) -> Day24RegVal {
    match r {
        "w" => Day24RegVal::W,
        "x" => Day24RegVal::X,
        "y" => Day24RegVal::Y,
        "z" => Day24RegVal::Z,
        _ => Day24RegVal::Value(r.parse().unwrap()),
    }
}

fn day24_read_data(filename: &str) -> Result<Vec<Day24Inst>, std::io::Error> {
    let data = std::fs::read_to_string(filename)?
        .lines()
        .map(|l| {
            let (i, rest) = l.split_once(" ").unwrap();
            let (a, b) = rest.split_once(" ").unwrap_or((rest, ""));
            match i {
                "inp" => Day24Inst::Inp(day24_parse_regval(a)),
                "add" => Day24Inst::Add(day24_parse_regval(a), day24_parse_regval(b)),
                "mul" => Day24Inst::Mul(day24_parse_regval(a), day24_parse_regval(b)),
                "div" => Day24Inst::Div(day24_parse_regval(a), day24_parse_regval(b)),
                "mod" => Day24Inst::Mod(day24_parse_regval(a), day24_parse_regval(b)),
                "eql" => Day24Inst::Eql(day24_parse_regval(a), day24_parse_regval(b)),
                _ => panic!("invalid input"),
            }
        })
        .collect();
    Ok(data)
}

fn day24_groups(data: &Vec<Day24Inst>) -> Vec<Vec<Day24Inst>> {
    data.linear_group_by(|_a, b| !matches!(b, Day24Inst::Inp(_)))
        .map(|g| g.to_vec())
        .collect()
}

#[derive(Default, Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Day24State {
    w: i64,
    x: i64,
    y: i64,
    z: i64,
}

fn day24_getreg(state: &Day24State, r: Day24RegVal) -> i64 {
    match r {
        Day24RegVal::W => state.w,
        Day24RegVal::X => state.x,
        Day24RegVal::Y => state.y,
        Day24RegVal::Z => state.z,
        Day24RegVal::Value(v) => v,
    }
}

fn day24_setreg(state: &Day24State, r: Day24RegVal, v: i64) -> Day24State {
    match r {
        Day24RegVal::W => Day24State { w: v, ..*state },
        Day24RegVal::X => Day24State { x: v, ..*state },
        Day24RegVal::Y => Day24State { y: v, ..*state },
        Day24RegVal::Z => Day24State { z: v, ..*state },
        Day24RegVal::Value(_) => panic!("invalid input"),
    }
}

fn day24_eval(state: &Day24State, inst: Day24Inst, input: i64) -> Day24State {
    match inst {
        Day24Inst::Inp(r) => day24_setreg(state, r, input),
        Day24Inst::Add(a, b) => {
            day24_setreg(state, a, day24_getreg(state, a) + day24_getreg(state, b))
        }
        Day24Inst::Mul(a, b) => {
            day24_setreg(state, a, day24_getreg(state, a) * day24_getreg(state, b))
        }
        Day24Inst::Div(a, b) => {
            day24_setreg(state, a, day24_getreg(state, a) / day24_getreg(state, b))
        }
        Day24Inst::Mod(a, b) => {
            day24_setreg(state, a, day24_getreg(state, a) % day24_getreg(state, b))
        }
        Day24Inst::Eql(a, b) => day24_setreg(
            state,
            a,
            (day24_getreg(state, a) == day24_getreg(state, b)) as i64,
        ),
    }
}

fn day24_eval_group(state: &Day24State, insts: &Vec<Day24Inst>, input: i64) -> Day24State {
    insts.iter().fold(*state, |s, i| day24_eval(&s, *i, input))
}

#[derive(Clone, Eq, PartialEq, Hash, Debug)]
struct Day24PathState {
    state: Day24State,
    group: usize,
    input: i64,
}

fn day24_succ(data: &Vec<Vec<Day24Inst>>, ps: &Day24PathState) -> Vec<(Day24PathState, i64)> {
    let mut ret = Vec::new();
    if ps.group >= data.len() {
        return ret;
    }
    let hasneg = data[ps.group]
        .iter()
        .any(|i| matches!(i, Day24Inst::Add(_, Day24RegVal::Value(x)) if *x < 0));
    for i in (1..=9).rev() {
        let state = day24_eval_group(&ps.state, &data[ps.group], i);
        if hasneg && state.x != 0 {
            // magic happens here
            continue;
        }
        ret.push((
            Day24PathState {
                state,
                group: ps.group + 1,
                input: (ps.input * 10) + i,
            },
            1,
        ));
    }
    ret
}

fn day24_goal(ps: &Day24PathState) -> bool {
    ps.group == 14 && ps.state.z == 0
}

#[inline(never)]
pub fn day24_puzzle1() -> Result<usize, std::io::Error> {
    let data = day24_read_data("inputs/input-24")?;
    let groups = day24_groups(&data);
    let (path, _cost) = astar(
        &Day24PathState {
            state: Day24State::default(),
            group: 0,
            input: 0,
        },
        |p| day24_succ(&groups, p),
        |p| 99999999999999 - p.input,
        |p| day24_goal(p),
    )
    .unwrap();
    let ret = path.last().unwrap().input;
    Ok(ret as usize)
}

#[inline(never)]
pub fn day24_puzzle2() -> Result<usize, std::io::Error> {
    let data = day24_read_data("inputs/input-24")?;
    let groups = day24_groups(&data);
    let (path, _cost) = astar(
        &Day24PathState {
            state: Day24State::default(),
            group: 0,
            input: 0,
        },
        |p| day24_succ(&groups, p),
        |p| p.input,
        |p| day24_goal(p),
    )
    .unwrap();
    let ret = path.last().unwrap().input;
    Ok(ret as usize)
}
