fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    // this consummes the iterator
    for i in v1_iter {
        println!("{}", i);
    }

    // if collect() is not called, this does nothing because
    // the iterator is lazy
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    assert_eq!(v2, vec![2, 3, 4]);

    // creating your own iterator
     struct Counter {
         count: u32,
     }

    impl Counter {
        fn new() -> Counter {
            Counter { count: 0 }
        }
    }

    impl Iterator for Counter {
        type Item = u32;

        fn next(&mut self) -> Option<Self::Item> {
            if self.count < 5 {
                self.count += 1;
                Some(self.count)
            } else {
                None
            }
        }
    }

    // now we can use all the iterator traits
    // because they are implemented using the next method
    let sum: u32 = Counter::new()
        .zip(Counter::new().skip(1))
        .map(|(a, b)| a * b)
        .filter(|x| x % 3 == 0)
        .sum();
    assert_eq!(18, sum);
}
