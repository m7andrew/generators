#![feature(coroutines, coroutine_trait, try_trait_v2)]
use generators::*;


#[test]
fn basic() {

	#[generator]
	fn numbers() -> i32 {
		yield 1;
		yield 2;
		yield 3;
	}

	let result: Vec<i32> = numbers().collect();
	assert_eq!(result, vec![1, 2, 3]);
}


#[test]
fn returns() {

	#[generator]
	fn numbers() -> i32 {
		yield 1;
		yield 2;
		return 3;
	}

	let result: Vec<i32> = numbers().collect();
	assert_eq!(result, vec![1, 2, 3]);
}


#[test]
fn try_expression() {

	fn two() -> Result<i32, i32> {
		Err(2)
	}

	#[generator]
	fn numbers() -> Result<i32, i32> {
		yield Ok(1);
		yield Ok(two()?);
		yield Ok(3);
	}

	let result: Vec<Result<i32, i32>> = numbers().collect();
	assert_eq!(result, vec![Ok(1), Err(2)]);
}


#[test]
fn recursive() {

	#[generator(boxed)]
	fn fib(start: i32, acc: i32) -> i32 {
		yield start + acc;
		for n in fib(acc, start + acc) { yield n }
	}

	let result: Vec<i32> = fib(1, 1).take(3).collect();
	assert_eq!(result, vec![2, 3, 5]);
}