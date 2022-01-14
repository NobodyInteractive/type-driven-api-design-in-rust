use std::{time::Duration, thread::sleep};

const CLEAR: &str = "\x1B[2L\x1B[1;1H";
struct Progress<Iter> {
    iter: Iter,
    i: usize
}

impl<Iter> Progress<Iter> {
    pub fn new(iter: Iter) -> Self {
        Progress{ iter, i: 0 }
    }
}

impl<Iter> Iterator for Progress<Iter>
where Iter: Iterator {
    type Item = Iter::Item;

    fn next(&mut self) -> Option<Self::Item> {
        println!("{}{}", CLEAR, "*".repeat(self.i));
        self.i += 1;
        self.iter.next()
    }

}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self>;
}

impl<Iter> ProgressIteratorExt for Iter {
    fn progress(self) -> Progress<Self>{
        Progress::new(self)
    }
}

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}

pub fn v5_main() {
    for n in (0 .. ).progress() {
        expensive_calculation(&n);
    }

    let v = vec![1, 2, 3];
    for n in v.iter().progress() {
        expensive_calculation(n);
    }

}

#[test]
fn v5_test()
{
    v5_main();
}