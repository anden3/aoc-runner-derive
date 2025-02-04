#![recursion_limit = "128"]

extern crate aoc_runner_internal;
extern crate core;
extern crate proc_macro;
extern crate proc_macro2;
extern crate quote;
extern crate syn;

mod generator;
mod map;
mod out;
mod runner;
mod types;
mod utils;

use crate::map::Map;
use proc_macro as pm;

thread_local! {
    static AOC_RUNNER: Map = Map::new();
}

#[proc_macro_attribute]
/// # Solution meta
///
/// Use this to flag a function as a solution for a given day :
/// `#[aoc(day1, part1)]`
///
/// You can also add a custom name to the function :
/// `#[aoc(day1, part1, Bytes)]`, it's useful to have multiple solutions to a given day & part and compare them !
///
/// The function must take a single parameter : a `&str` or a `&[u8]`, unless you use a [generator]
/// and return any type implementing `Display`.
///
/// ## Results & Options
///
/// Since 0.2.0, you can output `Result` & `Option` from solution function, with the following constraints :
///  - the output type must be named `Result` or `Option`, `type CustomResult<T> = Result<T, CustomError>;` cannot be used in return position.
///  - the first generic parameter must implement `Display`
///  - for `Result`s, the error must implement `Into<std::error::Error>`
///
/// You still can use a path before the `Result`/`Option`, like this : `std::io::Result<i32>`
///
/// [generator]: attr.aoc_generator.html
pub fn aoc(args: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    runner::runner_impl(args, input)
}

#[proc_macro_attribute]
/// # Generator meta
///
/// Use a generator when you need to pre-process your input :
///
/// ## Usage
/// Generator meta have 3 forms :
///  - a generator for the whole day : `#[aoc_generator(day1)]`
///  - a generator for a single part : `#[aoc_generator(day1, part1)]`
///  - a generator for a single (named) solution: `#[aoc_generator(day1, part1, Bytes)]`
///
/// The function must take a single parameter : a `&str` or a `&[u8]`, and output any sized type.
///
/// The corresponding solutions now take any parameter for which `Borrow` is implemented.
///
/// ## Results & Options
///
/// Since 0.2.0, you can output `Result` & `Option` from generator function, with the following constraints :
///  - the output type must be named `Result` or `Option`, `type CustomResult<T> = Result<T, CustomError>;` cannot be used in return position.
///  - for `Result`s, the error must implement `Into<std::error::Error>`
///
/// You still can use a path before the `Result`/`Option`, like this : `std::io::Result<i32>`
///
/// ## Note
/// A generator must be declared before it's solutions.
///
pub fn aoc_generator(args: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    generator::generator_impl(args, input)
}

#[proc_macro]
/// # Library declaration
///
/// This macro must be at the end of lib.rs
///
/// ## Usage
/// `aoc_lib! { year = 2018 }`
pub fn aoc_lib(input: pm::TokenStream) -> pm::TokenStream {
    out::lib_impl(input)
}

#[proc_macro]
/// # Main declaration
///
/// This macro must be at the end of main.rs
///
/// ## Usage
/// `aoc_main` has 2 forms :
///  - as a standalone binary : `aoc_main! { year = 2018 }`
///  - as a link to a library : `aoc_main! { lib = advent_of_code_2018 }` (you must had `extern crate advent_of_code_2018;` before)
pub fn aoc_main(input: pm::TokenStream) -> pm::TokenStream {
    out::main_impl(input)
}
