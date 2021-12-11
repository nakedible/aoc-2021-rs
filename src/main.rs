use std::cmp::{max, min};
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

fn day5_puzzle12(diag: bool) -> Result<usize, std::io::Error> {
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

fn day6_puzzle1() -> Result<usize, std::io::Error> {
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

fn day6_puzzle2() -> Result<usize, std::io::Error> {
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

fn day7_puzzle1() -> Result<usize, std::io::Error> {
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

fn day7_puzzle2() -> Result<usize, std::io::Error> {
    let crabs = std::fs::read_to_string("inputs/input-07")?
        .lines()
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let minfuel = (470..=475) // 0..2000
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

fn day8_puzzle1() -> Result<usize, std::io::Error> {
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

fn day8_puzzle2() -> Result<usize, std::io::Error> {
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

// fn day9_get_neighbours(&heightmap: [[i8; 100]; 100], row: usize, col: usize) -> [i8; 4] {

// }

fn day9_safeget(heightmap: &[[i8; 100]; 100], row: i64, col: i64) -> i8 {
    if row < 0 || row > 99 || col < 0 || col > 99 {
        127
    } else {
        heightmap[row as usize][col as usize]
    }
}

fn day9_puzzle1() -> Result<usize, std::io::Error> {
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

fn day9_puzzle2() -> Result<usize, std::io::Error> {
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

fn day10_puzzle1() -> Result<usize, std::io::Error> {
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

fn day10_puzzle2() -> Result<usize, std::io::Error> {
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

fn day11_puzzle1() -> Result<usize, std::io::Error> {
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

fn day11_puzzle2() -> Result<usize, std::io::Error> {
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
