use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

pub async fn process() {
    part_one().await;
    part_two().await;
}

fn score_one(line: &str) -> u32 {
    match line {
        "A X" => 3 + 1, // rock:rock
        "A Y" => 6 + 2, // rock:paper
        "A Z" => 0 + 3, // rock:scissor
        "B X" => 0 + 1, // paper:rock
        "B Y" => 3 + 2, // paper:paper
        "B Z" => 6 + 3, // paper:scissors
        "C X" => 6 + 1, // scissors:rock
        "C Y" => 0 + 2, // scissors:paper
        "C Z" => 3 + 3, // scissors:scissors
        _ => unreachable!(),
    }
}

fn score_two(line: &str) -> u32 {
    let newline = match line {
        "A X" => "A Z", // rock:lose:scissors
        "A Y" => "A X", // rock:draw:rock
        "A Z" => "A Y", // rock:win:paper
        "B X" => "B X", // paper:lose:rock
        "B Y" => "B Y", // paper:draw:paper
        "B Z" => "B Z", // paper:win:scissors
        "C X" => "C Y", // scissors:lose:paper
        "C Y" => "C Z", // scissors:draw:scissors
        "C Z" => "C X", // scissors:win:rock
        _ => unreachable!(),
    };
    score_one(newline)
}

async fn part_one() {
    let file = File::open("src/day_02/input.txt").await.unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut total_score: u32 = 0;
    while let Some(line) = lines.next_line().await.unwrap() {
        total_score += score_one(&line)
    }
    println!("Answer Part One: {}", total_score)
}

async fn part_two() {
    let file = File::open("src/day_02/input.txt").await.unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut total_score: u32 = 0;
    while let Some(line) = lines.next_line().await.unwrap() {
        total_score += score_two(&line)
    }
    println!("Answer Part Two: {}", total_score)
}
