# Ripin-rs

[![Crates.io](https://img.shields.io/crates/v/ripin.svg)](https://crates.io/crates/ripin)
[![Documentation](https://docs.rs/ripin/badge.svg)](https://docs.rs/ripin)
[![Build Status](https://travis-ci.org/Kerollmops/ripin-rs.svg)](https://travis-ci.org/Kerollmops/ripin-rs)
[![Coverage Status](https://coveralls.io/repos/github/Kerollmops/ripin-rs/badge.svg?branch=master)](https://coveralls.io/github/Kerollmops/ripin-rs?branch=master)

A library to handle [`Reverse Polish notated`](https://en.wikipedia.org/wiki/Reverse_Polish_notation) expressions.

Ripin can also evaluate variable expressions and is not limited to string tokens, it can handle any iterator type, the only limit is your imagination. There is [`examples`](https://github.com/Kerollmops/ripin-rs/tree/master/examples) to understand how to implement your own expression from custom types.

## Installation

Ripin is available on [crates.io](https://crates.io/crates/ripin) and can be included in your Cargo enabled project like this:

```toml
[dependencies]
ripin = "0.1"
```

Then include it in your code like this:

```rust
extern crate ripin;
```

## Examples

Ripin can evaluate [`Reverse Polish Notated`](https://en.wikipedia.org/wiki/Reverse_Polish_notation) expressions.

```rust
use ripin::expression::FloatExpr;

let expr_str = "3 4 + 2 *"; // (3 + 4) * 2
let tokens = expr_str.split_whitespace();

let expr = FloatExpr::<f32>::from_iter(tokens).unwrap();

println!("Expression {:?} gives {:?}", expr_str, expr.evaluate())
```

It is also possible to use variables in your expressions to make them more "variable".

```rust
use ripin::evaluate::VariableFloatExpr;
use ripin::variable::VarIdx;

let variables = vec![3.0, 500.0]; // Try changing variables here

let expr_str = "3 $1 + 2 * $0 -"; // (3 + $1) * 2 - $0
let tokens = expr_str.split_whitespace();

let expr = VariableFloatExpr::<f32, VarIdx>::from_iter(tokens).unwrap();

let result = expr.evaluate_with_variables::<usize, _>(variables);

println!("Expression {:?} gives {:?}", expr_str, result);
```
