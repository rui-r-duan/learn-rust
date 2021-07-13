use std::collections::HashMap;

pub struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    pub fn new(calculation: T) -> Cacher<T> {
	Cacher {
	    calculation,
	    value: HashMap::new(),
	}
    }

    pub fn value(&mut self, arg: u32) -> u32 {
	if self.value.contains_key(&arg) {
	    self.value.get(&arg).cloned().unwrap()
	}
	else {
	    let v = (self.calculation)(arg);
	    self.value.insert(arg, v);
	    v
	}
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
	let mut c = Cacher::new(|a| a);

	let _v1 = c.value(1);
	let v2 = c.value(2);

	assert_eq!(v2, 2);
    }
}
