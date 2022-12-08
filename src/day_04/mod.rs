use std::collections::HashSet;
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

pub async fn process() {
    part_one().await;
    part_two().await;
}

async fn part_one() {
    let file = File::open("src/day_04/input.txt").await.unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut total_score: u32 = 0;

    while let Some(line) = lines.next_line().await.unwrap() {
        let ranges: Vec<Vec<u32>> = line
            .split(",")
            .map(|s| s.split("-").map(|n| n.parse::<u32>().unwrap()).collect())
            .collect();

        // not the most memory efficient, but ranges were small and super easy to reason about
        let set_0: HashSet<u32> = HashSet::from_iter(ranges[0][0]..ranges[0][1] + 1);
        let set_1: HashSet<u32> = HashSet::from_iter(ranges[1][0]..ranges[1][1] + 1);

        let intersection: Vec<&u32> = set_0.intersection(&set_1).collect();

        let score = if intersection.len() == set_0.len() || intersection.len() == set_1.len() {
            1
        } else {
            0
        };

        // println!("{:?}, {:?}, {}", ranges[0], ranges[1], score);

        total_score += score;
    }

    println!("Answer Part One: {}", total_score)
}

async fn part_two() {
    let file = File::open("src/day_04/input.txt").await.unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut total_score: u32 = 0;

    while let Some(line) = lines.next_line().await.unwrap() {
        let ranges: Vec<Vec<u32>> = line
            .split(",")
            .map(|s| s.split("-").map(|n| n.parse::<u32>().unwrap()).collect())
            .collect();

        let set_0: HashSet<u32> = HashSet::from_iter(ranges[0][0]..ranges[0][1] + 1);
        let set_1: HashSet<u32> = HashSet::from_iter(ranges[1][0]..ranges[1][1] + 1);

        let intersection: Vec<&u32> = set_0.intersection(&set_1).collect();

        let score = if intersection.len() > 0 { 1 } else { 0 };

        println!("{:?}, {:?}, {}", ranges[0], ranges[1], score);

        total_score += score;
    }

    println!("Answer Part Two: {}", total_score)
}
