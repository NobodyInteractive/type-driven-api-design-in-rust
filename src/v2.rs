use std::{time::Duration, thread::sleep};

const CLEAR: &str = "\x1B[2L\x1B[1;1H";

// fn progress<T>(v: Vec<T>) {
//     let mut i = 1;
//     for n in v.iter() {
//         println!("{}{}", CLEAR, "*".repeat(i));
//         i += 1;
//         
//         expensive_calculation(n);  // Having an error mismatched type with <T>
//     }
// }

fn progress<T>(v: Vec<T>, f: fn(&T) -> ()) {
    let mut i = 1;
    for n in v.iter() {
        println!("{}{}", CLEAR, "*".repeat(i));
        i += 1;
        f(n); 
    }
}

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}

pub fn v2_main() {
    let v = vec![1, 2, 3];
    progress(v, expensive_calculation);
}

#[test]
fn v2_test()
{
    v2_main();
}