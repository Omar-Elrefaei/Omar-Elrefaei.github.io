use std::{
    fs::File,
    io::{BufRead, BufReader},
    iter::zip,
    path::Path,
};

use itertools::Itertools;

use rayon::prelude::*;

fn nums_from_file(filename: impl AsRef<Path>) -> Vec<i128> {
    let lines = BufReader::new(File::open(filename).expect("Error opening file")).lines();
    let nums = lines
        .map(|x| x.unwrap().parse::<i128>().expect("Integer parsing error"))
        .collect();
    nums
}

fn main() {
    let big_list = nums_from_file("input.txt");
    // let c0_big = crumble0(&big_list, 100);

    // checking optimized solution against the naive one
    // let c1_big = crumble1(&big_list, 100);
    // assert_eq!(c1_big, c0_big);

    let c2_big = crumble2(&big_list, 100);
    // assert_eq!(c2_big, c0_big);

    // parallel version has no return value, only prints
    // crumble3(&big_list, 100);
}

// Straightforward naive solution
fn crumble0(list: &Vec<i128>, window_size: usize) -> i32 {
    let mut n_crumbled = 0;
    // A list of target numbers, from the 100th item till the end
    let target_nums = &list[window_size..];
    // A sliding window iterator over the list of number
    let windows = list.windows(window_size);

    // loop over pairs of (target_num, and search_window)
    for (&num, window) in zip(target_nums, windows) {
        let mut found = false;

        for pair in window.iter().combinations(2) {
            if pair[0] + pair[1] == num {
                found = true;
                break;
            }
        }

        if !found {
            n_crumbled += 1;
            println!("{} is unsafe", num);
        }
    }
    n_crumbled
}

// Reduce the size of each window by stripping numbers that are too large
// to be added to anything before exceeding the target_num
fn crumble1(list: &Vec<i128>, window_size: usize) -> i32 {
    let mut n_crumbled = 0;
    let target_nums = &list[window_size..];
    let windows = list.windows(window_size);

    for (&num, window) in zip(target_nums, windows) {
        let mut found = false;

        let smallest = *window.iter().min().unwrap();
        let window_filtered = window.iter().filter(|&&x| x <= (num - smallest));

        for pair in window_filtered.combinations(2) {
            if pair[0] + pair[1] == num {
                found = true;
                break;
            }
        }

        if !found {
            n_crumbled += 1;
            println!("{} is unsafe", num);
        }
    }
    n_crumbled
}

// For this specific dateset, the pair of smallest and largest number in each window
// (after stripping away ones that are too large) seems to satisfy our condition majority
// of the time. Exploiting that with an early `continue` yields more than 10x performance.
fn crumble2(list: &Vec<i128>, window_size: usize) -> i32 {
    let mut n_crumbled = 0;
    let target_nums = &list[window_size..];
    let windows = list.windows(window_size);

    for (&num, window) in zip(target_nums, windows) {
        let mut found = false;

        let smallest = *window.iter().min().unwrap();
        let window_filtered = window.iter().filter(|&&x| x <= (num - smallest));
        let largest = window_filtered.clone().max().copied().unwrap_or_default();

        if (smallest + largest == num) && (smallest != largest) {
            continue;
        }

        for pair in window_filtered.combinations(2) {
            if pair[0] + pair[1] == num {
                found = true;
                break;
            }
        }

        if !found {
            n_crumbled += 1;
            println!("{} is unsafe", num);
        }
    }
    n_crumbled
}

// Multi-threaded version of crumble3 using Rayon
// For simplicity, I skipped making n_crumbled thread-safe.
// Correctness was confirmed by counting output lines.
fn crumble3(list: &Vec<i128>, window_size: usize) {
    let target_nums = &list[window_size..];
    let windows = list.par_windows(window_size);

    windows.zip(target_nums).for_each(|(window, &num)| {
        let window = window.iter();

        let smallest = *window.clone().min().unwrap();
        let window_filtered = window.filter(|&&x| x <= (num - smallest));
        let largest = window_filtered.clone().max().copied().unwrap_or_default();

        // we `return` early from the closure instead of using break/continue
        if (smallest + largest == num) && (smallest != largest) {
            return;
        }

        for pair in window_filtered.combinations(2) {
            if pair[0] + pair[1] == num {
                return;
            }
        }
        println!("{} is unsafe", num);
    });
}
