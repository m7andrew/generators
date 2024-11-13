#![allow(unused_braces)]

use std::ops::{ Coroutine, CoroutineState::* };
use std::pin::{ Pin };

//-----------------------------------------------------------------------------
//  Generator
//-----------------------------------------------------------------------------

pub struct Generator<C: Coroutine<Return = ()> + Unpin>(pub C);

//-----------------------------------------------------------------------------
//  Generator Iterator
//-----------------------------------------------------------------------------

impl<C: Coroutine<Return = ()> + Unpin> Iterator for Generator<C> {
	type Item = C::Yield;
	fn next(&mut self) -> Option<Self::Item> {
		match Pin::new(&mut self.0).resume(()) {
			Yielded(x)  => Some(x),
			Complete(_) => None,
		}
	}
}
