#![feature(test)]

extern crate test;

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn it_works() { // just testing the function insided a benchmark module
        assert_eq!(4, add_two(2));
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) { // using the test::Bencher to test the performance of the function
        b.iter(|| {
            for i in 1..1000 {
                add_two(2);
            }
        });
    }
}