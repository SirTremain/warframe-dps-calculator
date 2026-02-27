#![allow(dead_code, unused_variables, unused)]

// Demo code showing why bon, tap and rootcause might be useful.
use bon::Builder;
use rootcause::prelude::*;
use tap::{Pipe, Tap};

// BON PART
// So rust doesn't have positional arguments, which isn't a huge issue to be honest
// However, named arguments can make calling functions a bit more readable
// Have a look and tell me what you think
#[derive(Debug, Builder)]
struct CalcResult {
    pub result: i32,
    pub optional_result: Option<i32>,

    #[builder(default = 3)]
    pub default_result: i32,
}

fn demo_bon_struct() -> CalcResult {
    let minimal = CalcResult::builder().result(7).build();

    let _full = CalcResult::builder()
        .result(10)
        .optional_result(20) // <-- note how this doesn't have to be Some(20)
        .default_result(30)
        .build();

    minimal
}

#[bon::builder]
fn demo_bon_fn(
    thing: &str,
    other_thing: &str,
    third_thing: &str,
    optional_thing: Option<&str>,
) -> String {
    format!(
        "Thing: {thing} | Other Thing: {other_thing} | Third Thing: {third_thing} | Optional Thing: {optional_thing:?}"
    )
}

fn demo_bon_fn_builder() {
    let simple = demo_bon_fn()
        .thing("Hi")
        .other_thing("Hello")
        .third_thing("GG")
        .call();

    let fancy = demo_bon_fn()
        .thing("Hi")
        .other_thing("Hello")
        .third_thing("GG")
        .optional_thing("Optional")
        .call();
}

// TAP PART
// Tap is just a handy crate that people usally use for two functions:
// .tap() and .pipe().

// .tap() lets you inspect intermediate values mid-chain without breaking the chain
fn demo_tap() {
    let result: Vec<i32> = vec![5, 3, 8, 1, 9, 2, 7, 4, 6]
        .tap(|v| println!("  Input:    {v:?}"))
        .into_iter()
        .filter(|&x| x % 2 == 0)
        .collect::<Vec<_>>()
        .tap(|v| println!("  Evens:    {v:?}"))
        .into_iter()
        .map(|x| x * 10)
        .collect::<Vec<_>>()
        .tap(|v| println!("  ×10:      {v:?}"));

    println!("  Final:    {result:?}\n");

    // tap_mut — mutate a value in-place within a chain
    let data = vec![1, 2, 3]
        .tap_mut(|v| v.push(4))
        .tap_mut(|v| v.push(5))
        .tap(|v| println!("  After tap_mut pushes: {v:?}"));
    println!("  Data: {data:?}\n");
}

// .pipe() passes a value (by-value) into a closure and returns whatever
// the closure returns. It's useful for chaining function calls without
// breaking the chain.
fn demo_pipe() {
    let result = vec![1, 2, 3]
        .pipe(|v| v.into_iter())
        .filter(|&x| x % 2 == 0)
        .collect::<Vec<_>>();
    println!("  Pipe result: {result:?}\n");
}

// ROOTCAUSE PART
// Ok so rootcause is kind of a biggish one, since it has a large impact on the fn signatures
// The gist of it is that it attempts to make Rust error handling less verbose

// So we take a function that can return a Result usually, and replace the
// Result<Value, Error> with Result<Value, Report>
fn read_file(path: &str) -> Result<String, Report> {
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}

// Now if another functions calls read_file
fn load_config(path: &str) -> Result<String, Report> {
    let content = read_file(path).context("Falled to load config")?;
    // ^ We can attach context to any function that can return a Result
    Ok(content)
}

// For even more detail, this is how its chained
fn load_config_with_debug_info(path: &str) -> Result<String, Report> {
    let content = load_config(path)
        .attach(format!("Config path: {path}"))
        .attach("Expected format: TOML")?;

    Ok(content)
}

// Finally we wrap the whole thing in a main function to run it
// Run with cargo run -p calculations from the root of the project
fn main() -> Result<(), Report> {
    let config_path = "example.file";
    let environment = "dev";

    let _config = load_config_with_debug_info(config_path)
        .context("Application startup failed")
        .attach(format!("Environment: {environment}"))?;

    Ok(())
}
