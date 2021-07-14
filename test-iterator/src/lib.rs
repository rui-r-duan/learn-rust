//----------------------------------------------------------------
// Shoe
//----------------------------------------------------------------
// trait `Debug` is needed for assert_eq!: E0277
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

//----------------------------------------------------------------
// Counter
//----------------------------------------------------------------
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

//----------------------------------------------------------------
// Unit tests
//----------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    use std::convert::TryInto;

    fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
        v.try_into().unwrap_or_else(|v: Vec<T>| {
            panic!("Expected a Vec of length {} but it was {}", N, v.len())
        })
    }

    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        // without "mut", error E0596, because calling next() requires
        // the object to be mutable
        let mut v1_iter = v1.iter();

        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }

    #[test]
    fn iterator_in_for_loop() {
	let v1 = vec![1, 2, 3];
        let v1_iter = v1.iter();
        let mut v2 = Vec::new();

        // The for loop takes ownership of v1_iter and makes it mutable.
        for e in v1_iter {
            v2.push(e);
        }

        assert_eq!(vec_to_array(v2), [&1, &2, &3]);
    }

    #[test]
    fn into_iter_in_for_loop() {
	let v1 = vec![1, 2, 3];
	let mut v2 = Vec::new();

	// into_iter() is called
	// into_iter(self) -> IterMut<'a, T>
	for e in v1 {
	    v2.push(e);
	}

	assert_eq!(vec_to_array(v2), [1, 2, 3]);
    }

    #[test]
    fn call_into_iter_in_for_loop() {
	let v1 = vec![1, 2, 3];
	let v1_iter = v1.into_iter(); // no "mut" is needed
	let mut v2 = Vec::new();

	for e in v1_iter {
	    v2.push(e);
	}

	assert_eq!(vec_to_array(v2), [1, 2, 3]);
    }

    #[test]
    fn test_iter_mut() {
	let x = &mut [1, 2, 4];

	for elem in x.iter_mut() {
	    *elem += 2;
	}

	assert_eq!(x, &[3, 4, 6]);
    }

    #[test]
    fn iterator_sum() {
        let v1 = vec![1, 2, 3];

        let v1_iter = v1.iter();

        let total: i32 = v1_iter.sum();
	// We aren't allowed to use v1_iter after the call to sum because sum takes
	// ownership of the iterator we call it on.

        assert_eq!(total, 6);
    }

    #[test]
    fn test_map() {
	let v1: Vec<i32> = vec![1, 2, 3];

	let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();

	assert_eq!(v2, vec![2, 3, 4]);

	assert_eq!(v1, vec![1, 2, 3]);
    }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

	// value moved when calling shoes_in_size()
	// however, type `Vec<Shoe>` does not implement the `Copy` trait
        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );

	// value borrowed here after move, not allowed: E0382!
	// println!("{}", shoes[1].style);
    }

    #[test]
    fn calling_next_directly() {
        let mut counter = Counter::new();

        assert_eq!(counter.next(), Some(1));
        assert_eq!(counter.next(), Some(2));
        assert_eq!(counter.next(), Some(3));
        assert_eq!(counter.next(), Some(4));
        assert_eq!(counter.next(), Some(5));
        assert_eq!(counter.next(), None);
    }

    #[test]
    fn using_other_iterator_trait_methods() {
        let sum: u32 = Counter::new()
            .zip(Counter::new().skip(1))
            .map(|(a, b)| a * b)
            .filter(|x| x % 3 == 0)
            .sum();
        assert_eq!(18, sum);
    }
}
