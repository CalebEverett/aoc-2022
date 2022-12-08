use std::collections::{BTreeMap, VecDeque};
use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

pub async fn process() {
    part_one().await;
    part_two().await;
}

async fn part_one() {
    let file = File::open("src/day_05/input.txt").await.unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut stacks: BTreeMap<i32, VecDeque<char>> = BTreeMap::new();
    for i in 1..10 {
        stacks.insert(i, VecDeque::new());
    }

    for _ in 1..9 {
        let line = lines.next_line().await.unwrap().unwrap();
        let mut offset = 1;
        for stack_number in 1..10 {
            let character = line.chars().nth(offset).unwrap();
            if character != ' ' {
                stacks.get_mut(&stack_number).unwrap().push_back(character);
            }
            offset += 4;
        }
    }
    for key in stacks.keys() {
        println!("{}, {:?}", key, stacks.get(key).unwrap())
    }
    println!("");

    lines.next_line().await.unwrap();
    lines.next_line().await.unwrap();

    while let Some(line) = lines.next_line().await.unwrap() {
        let moves: Vec<i32> = line
            .split(" ")
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let mut to_stack = stacks.get(&moves[2]).unwrap().clone();
        let mut from_stack = stacks.get(&moves[1]).unwrap().clone();
        for _ in 0..moves[0] {
            to_stack.push_front(from_stack.pop_front().unwrap())
        }
        stacks.insert(moves[2], to_stack).unwrap();
        stacks.insert(moves[1], from_stack).unwrap();
    }

    for key in stacks.keys() {
        println!("{}, {:?}", key, stacks.get(key).unwrap())
    }

    let mut answer = String::new();
    for s in stacks.values() {
        answer.push(*s.front().unwrap())
    }

    println!("Answer Part One: {}", answer)
}

async fn part_two() {
    let file = File::open("src/day_05/input.txt").await.unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut stacks: BTreeMap<i32, VecDeque<char>> = BTreeMap::new();
    for i in 1..10 {
        stacks.insert(i, VecDeque::new());
    }

    for _ in 1..9 {
        let line = lines.next_line().await.unwrap().unwrap();
        let mut offset = 1;
        for stack_number in 1..10 {
            let character = line.chars().nth(offset).unwrap();
            if character != ' ' {
                stacks.get_mut(&stack_number).unwrap().push_back(character);
            }
            offset += 4;
        }
    }
    for key in stacks.keys() {
        println!("{}, {:?}", key, stacks.get(key).unwrap())
    }
    println!("");

    lines.next_line().await.unwrap();
    lines.next_line().await.unwrap();

    while let Some(line) = lines.next_line().await.unwrap() {
        let moves: Vec<i32> = line
            .split(" ")
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let mut to_stack = stacks.get(&moves[2]).unwrap().clone();
        let mut from_stack = stacks.get(&moves[1]).unwrap().clone();
        let new_from_stack = from_stack.split_off(moves[0] as usize);

        for _ in 0..moves[0] {
            to_stack.push_front(from_stack.pop_back().unwrap())
        }
        stacks.insert(moves[2], to_stack).unwrap();
        stacks.insert(moves[1], new_from_stack).unwrap();
    }

    for key in stacks.keys() {
        println!("{}, {:?}", key, stacks.get(key).unwrap())
    }

    let mut answer = String::new();
    for s in stacks.values() {
        answer.push(*s.front().unwrap())
    }

    println!("Answer Part Two: {}", answer)
}
