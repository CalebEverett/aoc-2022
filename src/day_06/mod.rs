use std::collections::{HashSet, VecDeque};
use tokio::fs::read_to_string;

pub async fn process() {
    part_one().await;
    part_two().await;
}

async fn part_one() {
    let seq_len = 4;

    let line = read_to_string("src/day_06/input.txt").await.unwrap();
    let mut answer: u32 = 1;
    let mut seq: VecDeque<char> = VecDeque::new();
    for c in line.chars() {
        seq.push_front(c);
        if answer > seq_len - 1 {
            let set: HashSet<&char> = HashSet::from_iter(seq.iter());
            if set.len() == seq_len as usize {
                break;
            }
            seq.pop_back().unwrap();
        }
        answer += 1;
    }

    println!("seq {:?}", seq);

    println!("Answer Part One: {}", answer)
}

async fn part_two() {
    let seq_len = 14;

    let line = read_to_string("src/day_06/input.txt").await.unwrap();
    let mut answer: u32 = 1;
    let mut seq: VecDeque<char> = VecDeque::new();
    for c in line.chars() {
        seq.push_front(c);
        if answer > seq_len - 1 {
            let set: HashSet<&char> = HashSet::from_iter(seq.iter());
            if set.len() == seq_len as usize {
                break;
            }
            seq.pop_back().unwrap();
        }
        answer += 1;
    }

    println!("seq {:?}", seq);

    println!("Answer Part Two: {}", answer)
}
