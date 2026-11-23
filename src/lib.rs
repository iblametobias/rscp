//! Rust Competitive Programming Template
//!
//! Made by Tobiasz Perczak
//!
//! Start `main` with [`load_input!`] (or [`load_interactive!`]), read with `get!`,
//! `get_vec!`, the graph loaders, `get_line` and `get_bytes`, and print with `put!`,
//! `putln!`, `putv!` and `yn!`. Output is buffered and flushed automatically when
//! `main` ends. Malformed or missing input is undefined behavior.
#![allow(dead_code)]

use std::io::{BufWriter, StdoutLock, Write};

/// Current input buffer.
#[doc(hidden)]
pub static mut INP: &'static str = "";
/// Read position in `INP`.
#[doc(hidden)]
pub static mut POS: usize = 0;
/// Buffered stdout.
#[doc(hidden)]
pub static mut OUT: Option<BufWriter<StdoutLock<'static>>> = None;
/// Interactive mode: input is read line by line and `putln!` flushes.
#[doc(hidden)]
pub static mut INTERACTIVE: bool = false;

/// Flushes the output buffer when dropped (created by `load_input!` and `load_interactive!`).
#[doc(hidden)]
pub struct Flush;

impl Drop for Flush {
    fn drop(&mut self) {
        flush();
    }
}

/// Flushes the output buffer.
///
/// Needed only in interactive mode after `put!`; `putln!` flushes by itself there.
pub fn flush() {
    unsafe {
        if let Some(o) = (*(&raw mut OUT)).as_mut() {
            let _ = o.flush();
        }
    }
}

/// Flushes the output buffer in interactive mode, does nothing otherwise.
#[doc(hidden)]
pub fn flush_if_interactive() {
    unsafe {
        if INTERACTIVE {
            flush();
        }
    }
}

/// Reads the next input line into `INP` (interactive mode only), false on EOF.
#[doc(hidden)]
pub fn refill() -> bool {
    unsafe {
        if !INTERACTIVE {
            return false;
        }
        let mut s = String::new();
        if std::io::stdin().read_line(&mut s).unwrap() == 0 {
            return false;
        }
        INP = Box::leak(s.into_boxed_str());
        POS = 0;
        true
    }
}

/// Returns the next whitespace-separated token (empty at end of input).
#[doc(hidden)]
pub fn token() -> &'static str {
    unsafe {
        let mut i;
        loop {
            let inp: &'static str = INP;
            let b = inp.as_bytes();
            i = POS;
            while i < b.len() && b[i].is_ascii_whitespace() {
                i += 1;
            }
            if i < b.len() || !refill() {
                break;
            }
        }
        let inp: &'static str = INP;
        let b = inp.as_bytes();
        let s = i;
        while i < b.len() && !b[i].is_ascii_whitespace() {
            i += 1;
        }
        POS = i;
        &inp[s..i]
    }
}

/// Returns the rest of the current line, or the next line if the rest is blank.
///
/// Strips the trailing `\r` and spaces. Example: `let name = get_line();`
pub fn get_line() -> &'static str {
    unsafe {
        let mut inp: &'static str = INP;
        let mut i = POS;
        let mut j = i;
        while j < inp.len() && inp.as_bytes()[j] != b'\n' && inp.as_bytes()[j].is_ascii_whitespace()
        {
            j += 1;
        }
        if j == inp.len() || inp.as_bytes()[j] == b'\n' {
            i = (j + 1).min(inp.len());
            if i == inp.len() && refill() {
                inp = INP;
                i = 0;
            }
        }
        let b = inp.as_bytes();
        let s = i;
        while i < b.len() && b[i] != b'\n' {
            i += 1;
        }
        POS = if i < b.len() { i + 1 } else { i };
        inp[s..i].trim_end()
    }
}

/// Returns the next token as bytes, without allocating.
///
/// Example: `let s = get_bytes(); s[i] == b'#'`
pub fn get_bytes() -> &'static [u8] {
    token().as_bytes()
}

/// Types readable from the input: numbers, `String`, `char`, `bool` and tuples up to 6.
pub trait Get {
    /// Reads the next value from the input.
    fn get() -> Self;
}

macro_rules! impl_get {
    ($($t:ty)*) => {$(
        impl Get for $t {
            #[inline]
            fn get() -> Self {
                unsafe { token().parse().unwrap_unchecked() }
            }
        }
    )*};
}
impl_get!(i8 i16 i32 i64 i128 isize u8 u16 u32 u64 u128 usize f32 f64 char bool String);

macro_rules! impl_get_tuple {
    ($($n:ident)+) => {
        impl<$($n: Get),+> Get for ($($n,)+) {
            #[inline]
            fn get() -> Self {
                ($($n::get(),)+)
            }
        }
    };
}
impl_get_tuple!(A B);
impl_get_tuple!(A B C);
impl_get_tuple!(A B C D);
impl_get_tuple!(A B C D E);
impl_get_tuple!(A B C D E F);

/// Loads all of stdin and sets up buffered output.
///
/// Must be the first statement in `main`; output is flushed when `main` ends.
/// Example: `load_input!();`
#[macro_export]
macro_rules! load_input {
    () => {
        unsafe {
            $crate::INP = Box::leak(
                std::io::read_to_string(std::io::stdin())
                    .unwrap()
                    .into_boxed_str(),
            );
            $crate::OUT = Some(::std::io::BufWriter::with_capacity(
                1 << 16,
                ::std::io::stdout().lock(),
            ));
        }
        let _flush = $crate::Flush;
    };
}

/// Sets up interactive mode: input is read lazily, line by line, and `putln!` flushes.
///
/// Use instead of `load_input!` for problems where the judge answers your output.
/// Example: `load_interactive!();`
#[macro_export]
macro_rules! load_interactive {
    () => {
        unsafe {
            $crate::INTERACTIVE = true;
            $crate::OUT = Some(::std::io::BufWriter::with_capacity(
                1 << 16,
                ::std::io::stdout().lock(),
            ));
        }
        let _flush = $crate::Flush;
    };
}

/// Reads the next value, with the type from context or given explicitly.
///
/// Works for numbers, `String`, `char`, `bool` and tuples up to 6.
/// Example: `let n: usize = get!();`, `get!(usize)`, `get!((usize, i64))`
#[macro_export]
macro_rules! get {
    () => {
        $crate::Get::get()
    };
    ($t:ty) => {
        <$t as $crate::Get>::get()
    };
}

/// Reads `n` values of type `T` into a `Vec`, like `vec![x; n]`.
///
/// `T` can be a tuple, or `_` to infer it from context.
/// Example: `get_vec![usize; n]`, `get_vec![(usize, usize); m]`
#[macro_export]
macro_rules! get_vec {
    ($t:ty; $n:expr) => {
        (0..$n).map(|_| $crate::get!($t)).collect::<Vec<$t>>()
    };
}

/// Builds adjacency lists from `m` edges `a b` (1-indexed in the input).
fn build_graph(n: usize, m: usize, directed: bool) -> Vec<Vec<usize>> {
    let mut g = vec![Vec::new(); n];
    for _ in 0..m {
        let a = usize::get() - 1;
        let b = usize::get() - 1;
        g[a].push(b);
        if !directed {
            g[b].push(a);
        }
    }
    g
}

/// Builds `(to, w)` adjacency lists from `m` edges `a b w` (1-indexed in the input).
fn build_wgraph<W: Get + Copy>(n: usize, m: usize, directed: bool) -> Vec<Vec<(usize, W)>> {
    let mut g = vec![Vec::new(); n];
    for _ in 0..m {
        let a = usize::get() - 1;
        let b = usize::get() - 1;
        let w = W::get();
        g[a].push((b, w));
        if !directed {
            g[b].push((a, w));
        }
    }
    g
}

/// Reads an undirected graph: `m` edges `a b`, returns 0-indexed adjacency lists.
///
/// Example: `let g = get_graph(n, m);`
pub fn get_graph(n: usize, m: usize) -> Vec<Vec<usize>> {
    build_graph(n, m, false)
}

/// Reads a directed graph: `m` edges `a b`, returns 0-indexed adjacency lists.
///
/// Example: `let g = get_digraph(n, m);`
pub fn get_digraph(n: usize, m: usize) -> Vec<Vec<usize>> {
    build_graph(n, m, true)
}

/// Reads a weighted undirected graph: `m` edges `a b w`, returns `(to, w)` lists.
///
/// Example: `let g = get_wgraph::<i64>(n, m);`
pub fn get_wgraph<W: Get + Copy>(n: usize, m: usize) -> Vec<Vec<(usize, W)>> {
    build_wgraph(n, m, false)
}

/// Reads a weighted directed graph: `m` edges `a b w`, returns `(to, w)` lists.
///
/// Example: `let g = get_wdigraph::<i64>(n, m);`
pub fn get_wdigraph<W: Get + Copy>(n: usize, m: usize) -> Vec<Vec<(usize, W)>> {
    build_wgraph(n, m, true)
}

/// Reads a tree on `n` vertices: `n - 1` edges `a b`, returns 0-indexed adjacency lists.
///
/// Example: `let t = get_tree(n);`
pub fn get_tree(n: usize) -> Vec<Vec<usize>> {
    build_graph(n, n.saturating_sub(1), false)
}

/// Reads a weighted tree on `n` vertices: `n - 1` edges `a b w`, returns `(to, w)` lists.
///
/// Example: `let t = get_wtree::<i64>(n);`
pub fn get_wtree<W: Get + Copy>(n: usize) -> Vec<Vec<(usize, W)>> {
    build_wgraph(n, n.saturating_sub(1), false)
}

/// Buffered `print!`.
///
/// In interactive mode call `flush()` afterwards if no `putln!` follows.
/// Example: `put!("{} ", x);`
#[macro_export]
macro_rules! put {
    ($($arg:tt)*) => {
        unsafe {
            use ::std::io::Write;
            let _ = write!((*(&raw mut $crate::OUT)).as_mut().unwrap_unchecked(), $($arg)*);
        }
    };
}

/// Buffered `println!`; also flushes in interactive mode.
///
/// Example: `putln!("{}", ans);`
#[macro_export]
macro_rules! putln {
    ($($arg:tt)*) => {
        unsafe {
            use ::std::io::Write;
            let _ = writeln!((*(&raw mut $crate::OUT)).as_mut().unwrap_unchecked(), $($arg)*);
            $crate::flush_if_interactive();
        }
    };
}

/// Prints an iterable separated by spaces, then a newline.
///
/// Example: `putv!(&v);`, `putv!(&v, "\n");`
#[macro_export]
macro_rules! putv {
    ($it:expr) => {
        $crate::putv!($it, " ")
    };
    ($it:expr, $sep:expr) => {{
        let mut first = true;
        for x in $it {
            if !first {
                $crate::put!("{}", $sep);
            }
            first = false;
            $crate::put!("{}", x);
        }
        $crate::putln!();
    }};
}

/// Prints `YES` or `NO` followed by a newline.
///
/// Example: `yn!(ok);`, `yn!(ok, "Yes", "No");`
#[macro_export]
macro_rules! yn {
    ($c:expr) => {
        $crate::putln!("{}", if $c { "YES" } else { "NO" })
    };
    ($c:expr, $y:expr, $n:expr) => {
        $crate::putln!("{}", if $c { $y } else { $n })
    };
}

// Template ends here
