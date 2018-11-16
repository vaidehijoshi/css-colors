# css-colors

[![Build Status](https://travis-ci.com/vaidehijoshi/css-colors.svg?branch=master)](https://travis-ci.com/vaidehijoshi/css-colors) [![css-colors](https://docs.rs/css-colors/badge.svg)](https://docs.rs/css-colors)

A Rust converter to transform CSS colors. 🎨

## Installation

Add the `css_colors` crate to your `Cargo.toml`'s list of dependencies:
```rust
[dependencies]
css_colors = "1.0"
```

Then tell your program to use the crate by adding the `extern crate` declaration to your root:
```rust
extern crate css_colors;
```

## Usage

The goal of this crate is to make it easy for you to transform and convert between common CSS color representations, allowing you to perform operations on your colors in the most intutive way possible. 🌈

### What is css_colors?

This crate allows you to create and convert between different color models. Currently, it handles transformation between the RGB color model and the HSL color model used in CSS3.

The RGB color model is often useful when you'd like to represent a color using a certain amount of red, green, and blue.
```css
background-color: rgb(255, 99, 71); // tomato
```
However, it is also possible to represent the same color using the HSL color model, which specifies the hue, saturation, and luminosity of a color:
```css
background-color: hsl(9, 100%, 64%); // also tomato!
```

You can also use CSS preprocessors like [Less](http://lesscss.org) to manipulate these colors in interesting ways.
```css
$tomato: hsl(9, 100%, 64%); // equivalent to rgb(255, 99, 71)
$dark-tomato: darken($tomato, 20%); // hsl(9, 100%, 44%)
$desaturated-tomato: desaturate($tomato, 40%); // hsl(9, 60%, 64%)
```

This crate allows you to perform operations that map to Less' [color operations API](http://lesscss.org/functions/#color-operations). These operations can be applied on both RGB & HSL color models.

### Examples

Represent colors as a valid CSS string:
```rust
use css_colors::{Color, rgb, hsla};

let salmon = rgb(250, 128, 114);
let chartreuse = hsla(90, 100, 50, 1.0);

assert_eq!(salmon.to_css(), "rgb(250, 128, 114)");
assert_eq!(chartreuse.to_css(), "hsla(90, 100%, 50%, 1.00)");
```

Convert between different color model representations:
```rust
use css_colors::{Color, rgb, rgba, hsl, hsla};

let chartreuse = rgb(127, 255, 0);

assert_eq!(chartreuse.to_hsl(), hsl(90, 100, 50));
assert_eq!(chartreuse.to_hsla(), hsla(90, 100, 50, 1.0));
assert_eq!(chartreuse.to_rgba(), rgba(127, 255, 0, 1.0));
```

Manipulate single colors to create new color model representations:
```rust
use css_colors::{Color, hsl, percent};

let chartreuse = hsl(90, 100, 50);

assert_eq!(chartreuse.darken(percent(20)), hsl(90, 100, 30));
assert_eq!(chartreuse.desaturate(percent(20)), hsl(90, 80, 50));
assert_eq!(chartreuse.greyscale(), hsl(90, 0, 50));
```

Manipulate multiple colors to create new color model representations:
```rust
use css_colors::{Color, rgb, rgba, hsl, hsla, percent};

let chartreuse = hsl(90, 100, 50);
let red = rgba(100, 0, 0, 1.0);

assert_eq!(
    chartreuse.mix(red, percent(50)).to_css(),
    "hsla(67, 98%, 25%, 1.00)"
);
assert_eq!(chartreuse.tint(percent(50)).to_css(), "hsl(90, 100%, 75%)");
assert_eq!(chartreuse.shade(percent(50)).to_css(), "hsl(90, 98%, 25%)");
```

Check out the [documentation](https://docs.rs/css-colors) to learn more about what color operations are available to use!

## Helpful Links

The following links may be helpful while using this crate.

1. CSS3's [RGB color model](https://www.w3.org/TR/css-color-3/#rgb-color)
2. CSS3's [HSL color model](https://www.w3.org/TR/css-color-3/#hsl-color)
3. Less' [color operation functions](http://lesscss.org/functions/#color-operations)

## Contributing

### Installation

* `git clone <repository-url>`
* `cd css-colors`
* `rustup update`
* `cargo build`

### Linting + plugins

Please use the below plugins to ensure code consistency when contributing to this crate.
* [Rustfmt](https://github.com/rust-lang-nursery/rustfmt) for formatting code style
* [RLS-vscode](https://github.com/rust-lang-nursery/rls-vscode) for RLS-based linting for VSCode
* [CodeLLDB](https://github.com/vadimcn/vscode-lldb) for debugging

### Building + testing

* `rustup update` – Updates to the most current Rust version
* `cargo build` – Builds the crate
* `cargo test` – Runs the test suite

We run our test suite against the Rust stable, beta, and nightly versions on Travis CI.

## License

This project is licensed under the [ISC License](LICENSE.md).
