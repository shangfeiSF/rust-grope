#![allow(dead_code)]

fn used_function() {}

fn unused_function() {}

fn noisy_unused_function() {}

fn main() {
    used_function();
}