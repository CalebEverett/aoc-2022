use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

pub async fn process() {
    part_one().await;
    part_two().await;
}

async fn part_one() {
    let file = File::open("src/day_01/input.txt").await.unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut max_calories: u32 = 0;
    let mut current_calories: u32 = 0;
    while let Some(line) = lines.next_line().await.unwrap() {
        if line == "" {
            if current_calories > max_calories {
                max_calories = current_calories
            }
            current_calories = 0;
        } else {
            current_calories += line.parse::<u32>().unwrap()
        }
    }
    println!("Answer Part One: {}", max_calories)
}

async fn part_two() {
    // make sure to add two line breaks at the end of file so the last
    // elf is calc'd.
    let file = File::open("src/day_01/input.txt").await.unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let mut max_calories: Vec<u32> = vec![0; 3];
    let mut current_calories: u32 = 0;
    while let Some(line) = lines.next_line().await.unwrap() {
        if line == "" {
            if current_calories > max_calories[0] {
                max_calories[2] = max_calories[1];
                max_calories[1] = max_calories[0];
                max_calories[0] = current_calories
            } else if current_calories > max_calories[1] {
                max_calories[2] = max_calories[1];
                max_calories[1] = current_calories
            } else if current_calories > max_calories[2] {
                max_calories[2] = current_calories
            }
            current_calories = 0;
        } else {
            current_calories += line.parse::<u32>().unwrap()
        }
    }
    println!("Answer Part Two: {}", max_calories.iter().sum::<u32>())
}
