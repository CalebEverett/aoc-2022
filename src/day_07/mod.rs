use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

pub async fn process() {
    part_one().await;
    part_two().await;
}

async fn part_one() {
    let file = File::open("src/day_07/input.txt").await.unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    let answer = 0;

    println!("Answer Part One: {}", answer)
}

async fn part_two() {
    unimplemented!()
}
