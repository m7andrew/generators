#![feature(coroutines, coroutine_trait)]

// Library Modules
mod generator;

// Library Exports
pub use generator::Generator;
pub use macros::generator;