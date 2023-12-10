use crate::utilities::{day::Day, error::AocResult, files::read_lines_from_file};

pub struct DayEight;

impl Day<u32, u32> for DayEight {
    fn part_1(input_file_name: &str) -> AocResult<u32> {
        let mut lines = read_lines_from_file(input_file_name)?;
        let mut trees: Vec<u32> = vec![];
        let mut cols: usize = 0;
        let mut visible_trees: u32 = 0;

        load_trees(&mut trees, &mut lines, &mut cols);

        let mut hidden: Vec<bool> = vec![true; trees.len()];

        process_visibility(&trees, &mut hidden, trees.len() / cols, cols);

        for is_hidden in hidden {
            if !is_hidden {
                visible_trees += 1;
            }
        }

        Ok(visible_trees)
    }

    fn part_2(input_file_name: &str) -> AocResult<u32> {
        let mut lines = read_lines_from_file(input_file_name)?;
        let mut trees: Vec<u32> = vec![];
        let mut cols: usize = 0;
        let mut highest_score: u32 = 0;

        load_trees(&mut trees, &mut lines, &mut cols);

        let mut scenic_scores = vec![1; trees.len()];

        process_scenic_score(&trees, &mut scenic_scores, trees.len() / cols, cols);

        for score in scenic_scores {
            if score > highest_score {
                highest_score = score;
            }
        }

        Ok(highest_score)
    }
}

fn line_to_u8_vec(line: &str) -> impl Iterator<Item = u32> + '_ {
    line.chars().map(|c| c.to_digit(10).unwrap())
}

fn load_trees(trees: &mut Vec<u32>, iter: &mut impl Iterator<Item = String>, cols: &mut usize) {
    for line in iter {
        let mut row: Vec<u32> = line_to_u8_vec(&line).collect();

        if *cols == 0 {
            *cols = row.len();
        }

        trees.append(&mut row);
    }
}

fn process_tree_visibility(
    index: usize,
    pos: usize,
    trees: &[u32],
    hidden: &mut [bool],
    tracker: &mut u32,
) {
    let current = trees[index];

    if pos == 0 || current > *tracker {
        hidden[index] = false;
    }

    if current > *tracker {
        *tracker = current;
    }
}

fn process_tree_scenic_score(
    index: usize,
    pos: usize,
    trees: &[u32],
    scores: &mut [u32],
    prev_trees: &mut Vec<u32>,
) {
    let current = trees[index];

    if pos == 0 {
        scores[index] = 0;
    } else {
        let mut visible_trees = 0;

        for tree in prev_trees.iter().rev() {
            visible_trees += 1;
            if *tree >= current {
                break;
            }
        }

        scores[index] *= visible_trees;
    }

    prev_trees.push(current);
}

fn process_visibility(trees: &[u32], hidden: &mut [bool], rows: usize, cols: usize) {
    let mut greatest = 0;

    for row in 0..rows {
        for col in 0..cols {
            process_tree_visibility(row * cols + col, col, trees, hidden, &mut greatest);
        }

        greatest = 0;

        for col in (0..cols).rev() {
            process_tree_visibility(
                row * cols + col,
                cols - col - 1,
                trees,
                hidden,
                &mut greatest,
            );
        }

        greatest = 0;
    }

    for col in 0..cols {
        for row in 0..rows {
            process_tree_visibility(row * cols + col, row, trees, hidden, &mut greatest);
        }

        greatest = 0;

        for row in (0..rows).rev() {
            process_tree_visibility(
                row * cols + col,
                rows - row - 1,
                trees,
                hidden,
                &mut greatest,
            );
        }

        greatest = 0;
    }
}

fn process_scenic_score(trees: &[u32], scores: &mut [u32], rows: usize, cols: usize) {
    let mut prev_trees: Vec<u32> = vec![];

    for row in 0..rows {
        for col in 0..cols {
            process_tree_scenic_score(row * cols + col, col, trees, scores, &mut prev_trees);
        }

        prev_trees = vec![];

        for col in (0..cols).rev() {
            process_tree_scenic_score(
                row * cols + col,
                cols - col - 1,
                trees,
                scores,
                &mut prev_trees,
            );
        }

        prev_trees = vec![]
    }

    for col in 0..cols {
        for row in 0..rows {
            process_tree_scenic_score(row * cols + col, row, trees, scores, &mut prev_trees);
        }

        prev_trees = vec![];

        for row in (0..rows).rev() {
            process_tree_scenic_score(
                row * cols + col,
                rows - row - 1,
                trees,
                scores,
                &mut prev_trees,
            );
        }

        prev_trees = vec![];
    }
}
