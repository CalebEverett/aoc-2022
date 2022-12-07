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
    let file = File::open("src/day_03/input.txt").await.unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut total_score: u32 = 0;

    while let Some(line) = lines.next_line().await.unwrap() {
        let chars: Vec<char> = line.chars().collect();
        let char_chunks = chars.split_at(chars.len() / 2);
        let ruck_0: HashSet<&char> = HashSet::from_iter(char_chunks.0.iter());

        for c in char_chunks.1 {
            if let Some(value) = ruck_0.get(c) {
                let mut score = **value as u32;
                if score > 96 {
                    score -= 96
                } else {
                    score -= 38
                }
                total_score += score;
                break;
            }
        }
    }
    println!("Answer Part One: {}", total_score)
}

async fn part_two() {
    let file = File::open("src/day_03/input.txt").await.unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut total_score: u32 = 0;

    while let (Some(line_0), Some(line_1), Some(line_2)) = (
        lines.next_line().await.unwrap(),
        lines.next_line().await.unwrap(),
        lines.next_line().await.unwrap(),
    ) {
        let ruck_0: HashSet<char> = HashSet::from_iter(line_0.chars().collect::<Vec<char>>());
        let ruck_1: HashSet<char> = HashSet::from_iter(line_1.chars().collect::<Vec<char>>());

        let inter: HashSet<&char> = HashSet::from_iter(ruck_0.intersection(&ruck_1));

        for c in line_2.chars() {
            if let Some(value) = inter.get(&&c) {
                let mut score = **value as u32;
                if score > 96 {
                    score -= 96
                } else {
                    score -= 38
                }
                total_score += score;
                break;
            }
        }
    }
    println!("Answer Part Two: {}", total_score)
}
