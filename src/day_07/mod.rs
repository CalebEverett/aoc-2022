use std::cell::RefCell;
use std::collections::BTreeMap;
use std::rc::Rc;

use tokio::{
    fs::File,
    io::{AsyncBufReadExt, BufReader},
};

// https://applied-math-coding.medium.com/a-tree-structure-implemented-in-rust-8344783abd75

struct TreeNode {
    name: String,
    parent: Option<Rc<RefCell<TreeNode>>>,
    size: u64,
    children: Option<BTreeMap<String, Rc<RefCell<TreeNode>>>>,
    level: usize,
}

impl TreeNode {
    fn new(
        name: &str,
        parent: Option<Rc<RefCell<TreeNode>>>,
        size: u64,
        level: usize,
        is_file: bool,
    ) -> Self {
        Self {
            name: name.to_string(),
            parent,
            size,
            level,
            children: (!is_file).then(|| BTreeMap::new()),
        }
    }

    pub fn add_child(
        &mut self,
        name: &str,
        parent: Rc<RefCell<TreeNode>>,
        size: u64,
        level: usize,
        is_file: bool,
    ) {
        let new_node = Rc::new(RefCell::new(TreeNode::new(
            name,
            Some(parent),
            size,
            level,
            is_file,
        )));
        if let Some(children) = &mut self.children {
            children.insert(name.to_string(), new_node);
        }
    }

    pub fn calc_size(&self) -> u64 {
        let size = if let Some(children) = &self.children {
            children.values().fold(0, |mut sum, child| {
                sum += child.borrow().calc_size();
                sum
            })
        } else {
            self.size
        };
        size
    }

    pub fn update_size(&mut self) {
        self.size = self.calc_size();
        if let Some(children) = &self.children {
            for child in children.values() {
                child.borrow_mut().update_size()
            }
        }
    }
    pub fn agg_dir_size(&self, threshold: u64, mut sum: u64) -> u64 {
        if let Some(children) = &self.children {
            if self.size <= threshold {
                sum += self.size
            }
            for child in children.values() {
                sum += child.borrow().agg_dir_size(threshold, 0);
            }
        }
        // println!("sum {sum} {} {}", self.name, self.children.is_some());
        sum
    }
    pub fn get_del_dir(&self, threshold: u64, mut size: u64) -> u64 {
        if let Some(children) = &self.children {
            if self.size >= threshold && self.size < size {
                size = self.size
            }
            for child in children.values() {
                size = child.borrow().get_del_dir(threshold, size);
            }
        }
        size
    }
}

impl std::fmt::Display for TreeNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f, "{}{}: {}", "  ".repeat(self.level), self.name, self.size)?;
        if let Some(children) = &self.children {
            for child in children.values() {
                write!(f, "{}", child.borrow())?;
            }
        }

        Ok(())
    }
}

pub async fn process() {
    let file = File::open("src/day_07/input.txt").await.unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    let root = Rc::new(RefCell::new(TreeNode::new("/", None, 0, 0, false)));
    let mut parent = Rc::clone(&root);

    lines.next_line().await.unwrap();

    let mut line_count = 0;

    while let Some(line) = lines.next_line().await.unwrap() {
        // println!("{}", line);

        line_count += 1;
        if line_count == 1000 {
            break;
        }
        let args: Vec<&str> = line.split(" ").collect();

        match args[0] {
            "dir" => {
                let level = parent.borrow().level + 1;
                parent
                    .borrow_mut()
                    .add_child(args[1], Rc::clone(&parent), 0, level, false)
            }
            "$" => match args[1] {
                "cd" => match args[2] {
                    ".." => {
                        let grandparent = Rc::clone(&parent);
                        parent = Rc::clone(grandparent.borrow().parent.as_ref().unwrap());
                    }
                    _ => {
                        let child = Rc::clone(&parent);
                        parent = Rc::clone(
                            child
                                .borrow()
                                .children
                                .as_ref()
                                .unwrap()
                                .get(args[2])
                                .unwrap(),
                        );
                    }
                },
                _ => continue,
            },

            _ => {
                let level = parent.borrow().level + 1;
                parent.borrow_mut().add_child(
                    args[1],
                    Rc::clone(&parent),
                    args[0].parse::<u64>().unwrap(),
                    level,
                    true,
                )
            }
        }
    }
    {
        root.borrow_mut().update_size()
    }
    println!(
        "Answer Part One: {}\n\n{}",
        root.borrow().agg_dir_size(100000, 0),
        root.borrow()
    );

    // part two
    let min_size_req = 30000000 - (70000000 - root.borrow().size);

    println!(
        "Answer Part Two: {}",
        root.borrow().get_del_dir(min_size_req, root.borrow().size),
    );
}
