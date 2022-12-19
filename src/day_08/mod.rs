use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

pub async fn process() {
    part_one().await;
    part_two().await;
}

async fn part_two() {
    let file = File::open("src/day_08/input.txt").await.unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut rows: Vec<Vec<u8>> = Vec::new();
    while let Some(line) = lines.next_line().await.unwrap() {
        let mut row: Vec<u8> = Vec::new();
        line.chars()
            .for_each(|c| row.push(c.to_digit(10).unwrap() as u8));
        rows.push(row);
    }

    let mut state_rows: Vec<Vec<u32>> = Vec::new();

    let max_row: usize = rows.len();
    let max_col: usize = rows[0].len();

    let mut answer = 0;
    for row in 0..max_row {
        state_rows.push(vec![0; max_col]);
        for col in 0..max_col {
            let val = rows[row][col];

            let mut left_score: u32 = 0;
            for left in (0..col).rev() {
                left_score += 1;
                if rows[row][left] >= val {
                    break;
                }
            }

            let mut right_score: u32 = 0;
            for right in (col + 1)..max_col {
                right_score += 1;
                if rows[row][right] >= val {
                    break;
                }
            }

            let mut up_score: u32 = 0;
            for up in (0..row).rev() {
                up_score += 1;
                if rows[up][col] >= val {
                    break;
                }
            }

            let mut down_score: u32 = 0;
            for down in (row + 1)..max_row {
                down_score += 1;
                if rows[down][col] >= val {
                    break;
                }
            }
            let score = left_score * right_score * up_score * down_score;
            // println!(
            //     "{} {} {} {} {}",
            //     left_score, up_score, right_score, down_score, score
            // );
            state_rows[row][col] = score;
            if score > answer {
                answer = score;
            }
        }
    }

    println!("Answer Part Two: {}", answer);
    // for row in state_rows {
    //     println!("{:?}", row);
    // }
}

async fn part_one() {
    let file = File::open("src/day_08/input.txt").await.unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let mut rows: Vec<Vec<u8>> = Vec::new();
    while let Some(line) = lines.next_line().await.unwrap() {
        let mut row: Vec<u8> = Vec::new();
        line.chars()
            .for_each(|c| row.push(c.to_digit(10).unwrap() as u8));
        rows.push(row);
    }
    let mut state_rows: Vec<Vec<u32>> = Vec::new();

    let max_row: usize = rows.len();
    let max_col: usize = rows[0].len();

    for row in 0..max_row {
        state_rows.push(vec![0; max_col]);

        let mut max_val = 0;
        for col in 0..max_col {
            if rows[row][col] > max_val || col == 0 {
                state_rows[row][col] = 1;
                max_val = rows[row][col];
            }
        }
        max_val = 0;
        for col in (0..max_col).rev() {
            if rows[row][col] > max_val || col == max_col - 1 {
                state_rows[row][col] = 1;
                max_val = rows[row][col];
            }
        }
    }

    for col in 0..max_row {
        let mut max_val = 0;
        for row in 0..max_row {
            if rows[row][col] > max_val || row == 0 {
                state_rows[row][col] = 1;
                max_val = rows[row][col];
            }
        }
        max_val = 0;
        for row in (0..max_row).rev() {
            if rows[row][col] > max_val || row == max_row - 1 {
                state_rows[row][col] = 1;
                max_val = rows[row][col];
            }
        }
    }

    let answer: u32 = state_rows.iter().fold(0, |mut sum, row| {
        sum += row.iter().sum::<u32>();
        sum
    });

    println!("Answer Part One: {:?}", answer);
    for row in state_rows {
        println!(
            "{:?}",
            row.iter()
                .map(|d| d.to_string())
                .collect::<Vec<String>>()
                .join("")
        );
    }
}
