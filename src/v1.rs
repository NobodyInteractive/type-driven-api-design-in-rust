use std::{time::Duration, thread::sleep};

const CLEAR: &str = "\x1B[2L\x1B[1;1H";

fn progress(v: Vec<i32>) {
    let mut i = 1;
    for n in v.iter() {
        println!("{}{}", CLEAR, "*".repeat(i));
        i += 1;
        expensive_calculation(n);
    }
}

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}

pub fn v1_main() {
    let v = vec![1, 2, 3];
    progress(v);
}

#[test]
fn v1_test()
{
    v1_main();
}