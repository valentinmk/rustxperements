// use std::mem;
#![feature(test)]
#![allow(unused_imports)]
extern crate rand;
extern crate test;
extern crate time;

use rand::Rng;
use test::Bencher;
use time::PreciseTime;

fn insertion_sort_naive(array: &mut Vec<i32>) {
    for (j, key) in array.clone().iter().enumerate() {
        if j == 0 { continue; }
        let mut i = (j - 1) as i32;
        while i >= 0 && array[i as usize] > *key {
            array[(i + 1) as usize] = array[i as usize];
            i = i - 1;
        }
        array[(i + 1) as usize] = *key;
    }
}

fn insertion_sort(array: &mut Vec<i32>) {
    for j in 1..array.len() {
        let mut i = j - 1;
        let buf = array[j];
        while i > 0 && array[i] > buf {
            array[i+1] = array[i];
            i = i - 1;
        }
        if i == 0 && array[i] > buf{
            array[i+1] = array[i];
            array[i] = buf;
        } else {
            array[i+1] = buf;
        }
    }
}

#[bench]
fn bench_sort_1_16(b: &mut Bencher) {
    let size_of_array = 819;
    let mut rng = rand::thread_rng();
    let mut test_array: Vec<i32> = (0..size_of_array).map(|_| rng.gen_range(0, size_of_array)).collect();
    b.iter(|| insertion_sort(&mut test_array));
}

#[bench]
fn bench_sort_2_64(b: &mut Bencher) {
    let size_of_array = 1638;
    let mut rng = rand::thread_rng();
    let mut test_array: Vec<i32> = (0..size_of_array).map(|_| rng.gen_range(0, size_of_array)).collect();
    b.iter(|| insertion_sort(&mut test_array));
}
#[bench]
fn bench_sort_3_128(b: &mut Bencher) {
    let size_of_array = 3276;
    let mut rng = rand::thread_rng();
    let mut test_array: Vec<i32> = (0..size_of_array).map(|_| rng.gen_range(0, size_of_array)).collect();
    b.iter(|| insertion_sort(&mut test_array));
}

#[bench]
fn bench_sort_naive_1_16(b: &mut Bencher) {
    let size_of_array = 819;
    let mut rng = rand::thread_rng();
    let mut test_array: Vec<i32> = (0..size_of_array).map(|_| rng.gen_range(0, size_of_array)).collect();
    b.iter(|| insertion_sort_naive(&mut test_array));
}

#[bench]
fn bench_sort_naive_2_64(b: &mut Bencher) {
    let size_of_array = 1638;
    let mut rng = rand::thread_rng();
    let mut test_array: Vec<i32> = (0..size_of_array).map(|_| rng.gen_range(0, size_of_array)).collect();
    b.iter(|| insertion_sort_naive(&mut test_array));
}
#[bench]
fn bench_sort_naive_3_128(b: &mut Bencher) {
    let size_of_array = 3276;
    let mut rng = rand::thread_rng();
    let mut test_array: Vec<i32> = (0..size_of_array).map(|_| rng.gen_range(0, size_of_array)).collect();
    b.iter(|| insertion_sort_naive(&mut test_array));
}

fn main() {
    let size_of_array = 13107;
    let mut rng = rand::thread_rng();
    let mut test_array: Vec<i32> = (0..size_of_array).map(|_| rng.gen_range(0, size_of_array)).collect();
    // Test time
    let start = PreciseTime::now();
    insertion_sort(&mut test_array);
    let end = PreciseTime::now();
    println!("Exec time = {:?}", start.to(end));
    // Test time
    let mut test_array: Vec<i32> = (0..size_of_array).map(|_| rng.gen_range(0, size_of_array)).collect();
    let start = PreciseTime::now();
    insertion_sort_naive(&mut test_array);
    let end = PreciseTime::now();
    println!("Exec naive time = {:?}", start.to(end));
}
