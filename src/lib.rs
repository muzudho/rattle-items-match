//! Matching is not limited to character strings. I'm trying to make a game AI.  
//! 文字列に限らないマッチングです。 ゲームAIを作ろうとしています。  

// Publish:
//
// (1) `cargo test`
// (2) `cargo run --example example`
// (3) Open auto-generated log file. I check it.
// (4) Remove the log file.
// (5) Update `README.md`.
// (6) Version up on Cargo.toml.
// (7) `cargo doc --open`
// (8) Comit to Git-hub.
// (9) `cargo publish --dry-run`
// (10) `cargo publish`

pub mod actual_items;
pub mod actual_items_builder;
pub mod any;
pub mod any_builder;
pub mod expected_items;
pub mod expected_items_builder;
pub mod machine;
pub mod range_contains_max;
pub mod range_contains_max_builder;
pub mod repeat;
pub mod repeat_builder;

pub struct Machine {
    actual_index: usize,
    expected_index: usize,
}

pub struct ActualItemsBuilder<T> {
    items: Vec<T>,
}

pub struct ActualItems<T> {
    items: Vec<T>,
}

pub struct ExpectedItemsBuilder<T> {
    items: Vec<Controls<T>>,
}

pub struct ExpectedItems<T> {
    items: Vec<Controls<T>>,
}

#[derive(Clone)]
pub enum Expected<T> {
    Exact(T),
    Any(Any<T>),
    RangeContainsMax(RangeContainsMax<T>),
}
/// Controls item.  
#[derive(Clone)]
pub enum Controls<T> {
    Exact(T),
    Any(Any<T>),
    RangeContainsMax(RangeContainsMax<T>),
    Expected(Expected<T>),
    Repeat(Repeat<T>),
}

pub struct RangeContainsMaxBuilder<T> {
    min: Option<T>,
    max: Option<T>,
}

/// Specify by range. Includes maximum value.  
/// 範囲で指定。最大値を含みます。  
#[derive(Clone)]
pub struct RangeContainsMax<T> {
    min: Option<T>,
    max: Option<T>,
}

pub struct AnyBuilder<T> {
    items: Vec<T>,
}

#[derive(Clone)]
pub struct Any<T> {
    items: Vec<T>,
}

pub struct RepeatBuilder<T> {
    expected: Option<Box<Controls<T>>>,
    min: usize,
    max: usize,
}

#[derive(Clone)]
pub struct Repeat<T> {
    expected: Box<Controls<T>>,
    min: usize,
    max: usize,
    matched_length: usize,
}

pub enum MatchingResult {
    Ongoing,
    Matched,
    NotMatch,
}
