#![allow(non_snake_case,non_camel_case_types,dead_code)]

/*
    Fill in the polarity function below. Use as many helpers as you want.
    Test your code by running 'cargo test' from the tester_rs_simple directory.
    
*/
#[derive(Debug, Clone)]
struct SignRecord {
    pos_rows: Vec<i32>,
    neg_rows: Vec<i32>,
    pos_cols: Vec<i32>,
    neg_cols: Vec<i32>,
}

type Board = Vec<Vec<char>>;
type Constraints = Vec<(String, Vec<i32>)>;

fn get_pos(board: &Board, r: usize, c: usize) -> char {
    board[r][c]
}

fn place(board: &Board, r: usize, c: usize, value: char) -> Board {
    let mut new_board = board.clone();
    new_board[r][c] = value;
    new_board
}

fn adjustments(xs: &[i32], idx: usize, f: impl Fn(i32) -> i32) -> Vec<i32> {
    let mut result = xs.to_vec();
    result[idx] = f(result[idx]);
    result
}

fn check_con(rules: &Constraints, key: &str, fallback: Vec<i32>) -> Vec<i32> {
    for (k, v) in rules {
        if k == key {
            return v.clone();
        }
    }
    fallback
}

fn next_ind(r: usize, c: usize, num_rows: usize, num_cols: usize) -> usize {
    r * num_cols + c + 1
}

fn adjust_record(tracker: &SignRecord, r: usize, c: usize, pole: char) -> SignRecord {
    let mut new_tracker = tracker.clone();
    match pole {
        '+' => {
            new_tracker.pos_rows = adjustments(&new_tracker.pos_rows, r, |x| x + 1);
            new_tracker.pos_cols = adjustments(&new_tracker.pos_cols, c, |x| x + 1);
        }
        '-' => {
            new_tracker.neg_rows = adjustments(&new_tracker.neg_rows, r, |x| x + 1);
            new_tracker.neg_cols = adjustments(&new_tracker.neg_cols, c, |x| x + 1);
        }
        _ => {}
    }
    new_tracker
}

fn is_valid_check(tallies: &SignRecord, rules: &Constraints) -> bool {
    let left_rules = check_con(rules, "left", vec![]);
    let right_rules = check_con(rules, "right", vec![]);
    let top_rules = check_con(rules, "top", vec![]);
    let bottom_rules = check_con(rules, "bottom", vec![]);

    let valid_row_check = tallies
        .pos_rows
        .iter()
        .enumerate()
        .all(|(i, &val)| i >= left_rules.len() || left_rules[i] == -1 || val <= left_rules[i])
        && tallies
            .neg_rows
            .iter()
            .enumerate()
            .all(|(i, &val)| i >= right_rules.len() || right_rules[i] == -1 || val <= right_rules[i]);

    let valid_col_check = tallies
        .pos_cols
        .iter()
        .enumerate()
        .all(|(j, &val)| j >= top_rules.len() || top_rules[j] == -1 || val <= top_rules[j])
        && tallies
            .neg_cols
            .iter()
            .enumerate()
            .all(|(j, &val)| j >= bottom_rules.len() || bottom_rules[j] == -1 || val <= bottom_rules[j]);

    valid_row_check && valid_col_check
}

fn result_check(tallies: &SignRecord, rules: &Constraints) -> bool {
    let left_rules = check_con(rules, "left", vec![]);
    let right_rules = check_con(rules, "right", vec![]);
    let top_rules = check_con(rules, "top", vec![]);
    let bottom_rules = check_con(rules, "bottom", vec![]);

    let valid_row_match = tallies
        .pos_rows
        .iter()
        .enumerate()
        .all(|(i, &val)| i >= left_rules.len() || left_rules[i] == -1 || val == left_rules[i])
        && tallies
            .neg_rows
            .iter()
            .enumerate()
            .all(|(i, &val)| i >= right_rules.len() || right_rules[i] == -1 || val == right_rules[i]);

    let valid_col_match = tallies
        .pos_cols
        .iter()
        .enumerate()
        .all(|(j, &val)| j >= top_rules.len() || top_rules[j] == -1 || val == top_rules[j])
        && tallies
            .neg_cols
            .iter()
            .enumerate()
            .all(|(j, &val)| j >= bottom_rules.len() || bottom_rules[j] == -1 || val == bottom_rules[j]);

    valid_row_match && valid_col_match
}

fn check_placement(board: &Board, r: usize, c: usize, pole: char, rows: usize, cols: usize) -> bool {
    let adjacent = [
        (r.checked_sub(1), Some(c)),
        (Some(r + 1), Some(c)),
        (Some(r), c.checked_sub(1)),
        (Some(r), Some(c + 1)),
    ];

    adjacent.iter().all(|(nr, nc)| {
        nr.is_none()
            || nc.is_none()
            || nr.unwrap() >= rows
            || nc.unwrap() >= cols
            || get_pos(board, nr.unwrap(), nc.unwrap()) != pole
    })
}

fn place_horizontal(
    rules: &Constraints,
    original: &Board,
    answer: &Board,
    r: usize,
    tallies: &SignRecord,
    c: usize,
    rows: usize,
    cols: usize,
) -> Option<Board> {
    if c + 1 < cols
        && get_pos(answer, r, c) == 'X'
        && get_pos(answer, r, c + 1) == 'X'
        && get_pos(original, r, c) == 'L'
        && get_pos(original, r, c + 1) == 'R'
    {
        let temp1 = place(&place(answer, r, c, '+'), r, c + 1, '-');
        let t1 = adjust_record(&adjust_record(tallies, r, c, '+'), r, c + 1, '-');
        if check_placement(&temp1, r, c, '+', rows, cols)
            && check_placement(&temp1, r, c + 1, '-', rows, cols)
            && is_valid_check(&t1, rules)
        {
            if let Some(sol) = find_solution(
                next_ind(r, c, rows, cols),
                rows,
                cols,
                original,
                &temp1,
                &t1,
                rules,
            ) {
                return Some(sol);
            }
        }

        let temp2 = place(&place(answer, r, c, '-'), r, c + 1, '+');
        let t2 = adjust_record(&adjust_record(tallies, r, c, '-'), r, c + 1, '+');
        if check_placement(&temp2, r, c, '-', rows, cols)
            && check_placement(&temp2, r, c + 1, '+', rows, cols)
            && is_valid_check(&t2, rules)
        {
            if let Some(sol) = find_solution(
                next_ind(r, c, rows, cols),
                rows,
                cols,
                original,
                &temp2,
                &t2,
                rules,
            ) {
                return Some(sol);
            }
        }
    }
    None
}

fn place_vertical(
    original: &Board,
    rows: usize,
    rules: &Constraints,
    answer: &Board,
    r: usize,
    tallies: &SignRecord,
    c: usize,
    cols: usize,
) -> Option<Board> {
    if r + 1 < rows
        && get_pos(answer, r, c) == 'X'
        && get_pos(answer, r + 1, c) == 'X'
        && get_pos(original, r, c) == 'T'
        && get_pos(original, r + 1, c) == 'B'
    {
        let temp1 = place(&place(answer, r, c, '+'), r + 1, c, '-');
        let t1 = adjust_record(&adjust_record(tallies, r, c, '+'), r + 1, c, '-');
        if check_placement(&temp1, r, c, '+', rows, cols)
            && check_placement(&temp1, r + 1, c, '-', rows, cols)
            && is_valid_check(&t1, rules)
        {
            if let Some(sol) = find_solution(
                next_ind(r, c, rows, cols),
                rows,
                cols,
                original,
                &temp1,
                &t1,
                rules,
            ) {
                return Some(sol);
            }
        }

        let temp2 = place(&place(answer, r, c, '-'), r + 1, c, '+');
        let t2 = adjust_record(&adjust_record(tallies, r, c, '-'), r + 1, c, '+');
        if check_placement(&temp2, r, c, '-', rows, cols)
            && check_placement(&temp2, r + 1, c, '+', rows, cols)
            && is_valid_check(&t2, rules)
        {
            if let Some(sol) = find_solution(
                next_ind(r, c, rows, cols),
                rows,
                cols,
                original,
                &temp2,
                &t2,
                rules,
            ) {
                return Some(sol);
            }
        }
    }
    None
}

fn try_placements(
    idx: usize,
    rules: &Constraints,
    tallies: &SignRecord,
    answer: &Board,
    rows: usize,
    cols: usize,
    original: &Board,
) -> Option<Board> {
    let r = idx / cols;
    let c = idx % cols;

    if let Some(sol) = place_horizontal(rules, original, answer, r, tallies, c, rows, cols) {
        return Some(sol);
    }

    if let Some(sol) = place_vertical(original, rows, rules, answer, r, tallies, c, cols) {
        return Some(sol);
    }
    None
}

fn find_solution(
    idx: usize,
    rows: usize,
    cols: usize,
    original: &Board,
    answer: &Board,
    tallies: &SignRecord,
    rules: &Constraints,
) -> Option<Board> {
    if idx == rows * cols {
        return if result_check(tallies, rules) {
            Some(answer.clone())
        } else {
            None
        };
    }
    let r = idx / cols;
    let c = idx % cols;

    if get_pos(answer, r, c) != 'X' {
        return find_solution(idx + 1, rows, cols, original, answer, tallies, rules);
    }

    if let Some(sol) = try_placements(idx, rules, tallies, answer, rows, cols, original) {
        return Some(sol);
    }
    find_solution(idx + 1, rows, cols, original, answer, tallies, rules)
}

fn polarity(board: &[&str], specs: &(Vec<i32>, Vec<i32>, Vec<i32>, Vec<i32>)) -> Vec<String> {
    let (left, right, top, bottom) = specs;
    let rows = board.len();
    let cols = if rows == 0 { 0 } else { board[0].len() };

    let grid: Board = board
        .iter()
        .map(|&row| row.chars().collect())
        .collect();

    let rules: Constraints = vec![
        ("left".to_string(), left.clone()),
        ("right".to_string(), right.clone()),
        ("top".to_string(), top.clone()),
        ("bottom".to_string(), bottom.clone()),
    ];

    let initial_answer: Board = vec![vec!['X'; cols]; rows];
    let initial_tallies = SignRecord {
        pos_rows: vec![0; rows],
        neg_rows: vec![0; rows],
        pos_cols: vec![0; cols],
        neg_cols: vec![0; cols],
    };

    match find_solution(0, rows, cols, &grid, &initial_answer, &initial_tallies, &rules) {
        Some(solution) => solution
            .iter()
            .map(|row| row.iter().collect::<String>())
            .collect(),
        None => Vec::new(), 
    }
}
#[cfg(test)]
#[path = "tests.rs"]
mod tests;

