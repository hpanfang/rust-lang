#[cfg(test)]
mod tests {
	use super::*;

    #[test]
    fn mean_test() {
    	let v = vec![1, 2, 3, -5];
        assert_eq!(sets::mean(v), 0.25);
    }
    #[test]
    fn median_test() {
    	let v1 = vec![1, 2, 3, 4, 5];
    	let v2 = vec![1, 2, 3, 4, 5, 6];
    	assert_eq!(sets::median(v1), 3.0);
    	assert_eq!(sets::median(v2), 3.5);
    }
    #[test]
    fn mode_test() {
    	let v1 = vec![1, 2, 3, 3, 4, 5, 6];
    	assert_eq![sets::mode(v1), 3];
    }
}

pub mod sets {
	pub fn mean(list: Vec<i32>) -> f64  {
		let mut mean = 0;

        for i in &list {
            mean += i;
        }
        mean as f64 / list.len() as f64
	}

	pub fn median(list: Vec<i32>) -> f64 {
		let mid : usize = list.len() / 2;

        if list.len() % 2 == 1 {
            f64::from(list[mid])
        } else {
        	mean(list[mid-1..mid+1].to_vec()) 
        }
	}

	pub fn mode(list: Vec<i32>) -> i32 {
		use std::collections::HashMap;
        let mut count = HashMap::new();
        let mut mode = 0;

        for i in &list {
        	let current = count.entry(i).or_insert(1);
            *current += 1;
            if current > &mut mode {
            	mode = *current;
            }
        }
        mode
	}
}
