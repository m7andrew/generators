#![feature(coroutines, coroutine_trait)]
use generators::*;


#[generator]
fn simple() -> i32 {
	yield 1;
	yield 2;
	yield 3;
}


#[test]
fn test_simple() {
	let result: Vec<i32> = simple().collect();
	assert_eq!(result, vec![1, 2, 3]);
}