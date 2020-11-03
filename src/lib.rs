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

pub mod actual;
pub mod any;
pub mod expected;
pub mod machine;
pub mod range_includes_max;
pub mod repeat;

pub struct Machine<T> {
    actual: Option<ActualVal<T>>,
    expected: Option<ExpectedVal<T>>,
}

pub struct MachineState {
    actual_index: usize,
    expected_index: usize,
    is_final: bool,
    matched_length_in_repeat: usize,
    matched_length_in_seq: usize,
}

pub struct MachineVal<T> {
    actual: ActualVal<T>,
    expected: ExpectedVal<T>,
}

pub struct Actual<T> {
    items: Vec<T>,
}

#[derive(Clone)]
pub struct ActualVal<T> {
    items: Vec<T>,
}

pub struct Expected<T> {
    items: Vec<Controls<T>>,
}

#[derive(Clone)]
pub struct ExpectedVal<T> {
    items: Vec<Controls<T>>,
}

/// Controls item.  
/// 制御項目。  
#[derive(Clone)]
pub enum Controls<T> {
    Once(Quantity<T>),
    Repeat(RepeatVal<T>),
}
/// Quantity.  
/// 量。  
#[derive(Clone)]
pub enum Quantity<T> {
    One(Operand<T>),
    Any(AnyVal<T>),
}
/// Operand. Logical operator not included.  
/// 項。論理演算子は含みません。  
#[derive(Clone)]
pub enum Operand<T> {
    /// 1つだけのもの。  
    Pin(T),
    /// Sequence.  
    /// 連続のもの。  
    Seq(Vec<T>),
    /// 範囲で指定。  
    RangeIncludesMax(RangeIncludesMaxVal<T>),
}

pub struct RangeIncludesMax<T> {
    min: Option<T>,
    max: Option<T>,
}

/// Specify by range. Includes maximum value.  
/// 範囲で指定。最大値を含みます。  
#[derive(Clone)]
pub struct RangeIncludesMaxVal<T> {
    min: Option<T>,
    max: Option<T>,
}

pub struct Any<T> {
    items: Vec<Operand<T>>,
}

#[derive(Clone)]
pub struct AnyVal<T> {
    items: Vec<Operand<T>>,
}

pub struct Repeat<T> {
    quantity: Option<Box<Quantity<T>>>,
    min: usize,
    max_not_included: usize,
}

#[derive(Clone)]
pub struct RepeatVal<T> {
    quantity: Box<Quantity<T>>,
    min: usize,
    max_not_included: usize,
}

pub enum MatchingResult {
    Ongoing,
    Matched,
    NotMatch,
}
