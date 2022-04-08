use std::io::{self, BufRead, Write};

fn merge(first_array: &[i32], second_array: &[i32]) -> Vec<i32> {
    let mut resulting_array = Vec::with_capacity(first_array.len() + second_array.len());

    let mut i = 0;
    let mut j = 0;

    while i != first_array.len() || j != second_array.len() {
        if i < first_array.len() && (j >= second_array.len() || first_array[i] < second_array[j]) {
            resulting_array.push(first_array[i]);
            i += 1;
        } else {
            resulting_array.push(second_array[j]);
            j += 1;
        }
    }

    resulting_array
}

fn merge_sort(input: &[i32]) -> Vec<i32> {
    if input.len() <= 1 {
        return input.to_vec();
    }

    let input_length = input.len();

    let first_part = merge_sort(&input[0..input_length / 2]);
    let second_part = merge_sort(&input[input_length / 2..input_length]);

    merge(&first_part, &second_part)
}

fn main() {
    let io_handle = io::stdin();
    let input_lines = io_handle
        .lock()
        .lines()
        .collect::<io::Result<Vec<String>>>()
        .unwrap();

    if input_lines.len() < 2 {
        println!("Missing data!");
        return;
    }

    let input_size: usize = input_lines[0].parse().unwrap();
    let input_array: Vec<i32> = input_lines[1]
        .split(' ')
        .take(input_size)
        .map(|n| n.parse::<i32>().unwrap())
        .collect();

    let sorted_array = merge_sort(&input_array);
    let io_handle = io::stdout();

    let s = sorted_array
        .iter()
        .map(|i| i.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    io_handle.lock().write_all(s.as_bytes()).unwrap();
}

#[test]
fn test_sort_empty() {
    let input = [];

    assert_eq!(merge_sort(&input), []);
}

#[test]
fn test_sort_odd_size() {
    let input = [5, 8, 1];

    assert_eq!(merge_sort(&input), [1, 5, 8]);
}

#[test]
fn test_sort_even_size() {
    let input = [5, 8, 1, 0];

    assert_eq!(merge_sort(&input), [0, 1, 5, 8]);
}

#[test]
fn test_sort_one_element() {
    let input = [5];

    assert_eq!(merge_sort(&input), [5]);
}

#[test]
fn test_merge_both_emtpy() {
    let a = [];
    let b = [];

    assert_eq!(merge(&a, &b), []);
}

#[test]
fn test_one_emtpy() {
    let a = [1];
    let b = [];

    assert_eq!(merge(&a, &b), [1]);

    let a = [];
    let b = [2];

    assert_eq!(merge(&a, &b), [2]);
}

#[test]
fn test_same_size() {
    let a = [1];
    let b = [2];

    assert_eq!(merge(&a, &b), [1, 2]);
}

#[test]
fn test_different_size() {
    let a = [1, 2, 3];
    let b = [4, 5];

    assert_eq!(merge(&a, &b), [1, 2, 3, 4, 5]);

    let a = [1, 2];
    let b = [3, 4, 5];

    assert_eq!(merge(&a, &b), [1, 2, 3, 4, 5]);
}
