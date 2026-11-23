//! Rust Competitive Programming Template
//!
//! Made by Tobiasz Perczak

use std::io::{BufWriter, StdoutLock, Write};

#[doc(hidden)]
pub static mut INP: &'static str = "";
#[doc(hidden)]
pub static mut POS: usize = 0;
#[doc(hidden)]
pub static mut OUT: Option<BufWriter<StdoutLock<'static>>> = None;

/// Flushes the output buffer when dropped (created by `load_input!`).
#[doc(hidden)]
pub struct Flush;

impl Drop for Flush {
    fn drop(&mut self) {
        unsafe {
            if let Some(o) = (*(&raw mut OUT)).as_mut() {
                let _ = o.flush();
            }
        }
    }
}

/// Next whitespace-separated token.
#[doc(hidden)]
pub fn token() -> &'static str {
    unsafe {
        let inp: &'static str = INP;
        let b = inp.as_bytes();
        let mut i = POS;
        while i < b.len() && b[i].is_ascii_whitespace() {
            i += 1;
        }
        let s = i;
        while i < b.len() && !b[i].is_ascii_whitespace() {
            i += 1;
        }
        POS = i;
        &inp[s..i]
    }
}

/// Rest of the current line; if that is blank, the next line.
#[doc(hidden)]
pub fn line() -> &'static str {
    unsafe {
        let inp: &'static str = INP;
        let b = inp.as_bytes();
        let mut i = POS;
        let mut j = i;
        while j < b.len() && b[j] != b'\n' && b[j].is_ascii_whitespace() {
            j += 1;
        }
        if j == b.len() {
            i = j;
        } else if b[j] == b'\n' {
            i = j + 1;
        }
        let s = i;
        while i < b.len() && b[i] != b'\n' {
            i += 1;
        }
        POS = if i < b.len() { i + 1 } else { i };
        inp[s..i].trim_end()
    }
}

/// Types readable from the input: primitives, `String`, `char` and tuples.
#[doc(hidden)]
pub trait Get {
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

/// Reads `m` edges `a b` (1-indexed in the input), returns 0-indexed adjacency lists.
#[doc(hidden)]
pub fn read_graph(n: usize, m: usize, directed: bool) -> Vec<Vec<usize>> {
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

/// Reads `m` edges `a b w` (1-indexed in the input), returns 0-indexed `(to, w)` lists.
#[doc(hidden)]
pub fn read_wgraph<W: Get + Copy>(n: usize, m: usize, directed: bool) -> Vec<Vec<(usize, W)>> {
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

/// Next value (type from context), or `get!(T)`. Works for numbers, `String`, `char`,
/// `bool` and tuples up to 6: `get!((usize, i64))`.
#[macro_export]
macro_rules! get {
    () => {
        $crate::Get::get()
    };
    ($t:ty) => {
        <$t as $crate::Get>::get()
    };
}

/// `get_vec!(n)` (type from context) or `get_vec!(n, T)`. `T` can be a tuple:
/// `get_vec!(m, (usize, usize, i64))`.
#[macro_export]
macro_rules! get_vec {
    ($n:expr) => {
        (0..$n).map(|_| $crate::get!()).collect()
    };
    ($n:expr, $t:ty) => {
        (0..$n).map(|_| $crate::get!($t)).collect::<Vec<$t>>()
    };
}

/// Undirected graph: reads `m` edges `a b` (1-indexed), gives `Vec<Vec<usize>>` (0-indexed).
#[macro_export]
macro_rules! get_graph {
    ($n:expr, $m:expr) => {
        $crate::read_graph($n, $m, false)
    };
}

/// Directed graph: reads `m` edges `a b` (1-indexed), gives `Vec<Vec<usize>>` (0-indexed).
#[macro_export]
macro_rules! get_digraph {
    ($n:expr, $m:expr) => {
        $crate::read_graph($n, $m, true)
    };
}

/// Weighted undirected graph: `m` edges `a b w`, gives `Vec<Vec<(usize, W)>>`.
#[macro_export]
macro_rules! get_wgraph {
    ($n:expr, $m:expr, $w:ty) => {
        $crate::read_wgraph::<$w>($n, $m, false)
    };
}

/// Weighted directed graph: `m` edges `a b w`, gives `Vec<Vec<(usize, W)>>`.
#[macro_export]
macro_rules! get_wdigraph {
    ($n:expr, $m:expr, $w:ty) => {
        $crate::read_wgraph::<$w>($n, $m, true)
    };
}

/// Tree on `n` vertices: reads `n - 1` edges `a b` (1-indexed), gives `Vec<Vec<usize>>`.
#[macro_export]
macro_rules! get_tree {
    ($n:expr) => {{
        let n: usize = $n;
        $crate::read_graph(n, n.saturating_sub(1), false)
    }};
}

/// Weighted tree on `n` vertices: `n - 1` edges `a b w`, gives `Vec<Vec<(usize, W)>>`.
#[macro_export]
macro_rules! get_wtree {
    ($n:expr, $w:ty) => {{
        let n: usize = $n;
        $crate::read_wgraph::<$w>(n, n.saturating_sub(1), false)
    }};
}

/// Rest of the current line (or the next one if the current is blank), `&'static str`.
#[macro_export]
macro_rules! get_line {
    () => {
        $crate::line()
    };
}

/// Next token as `&'static [u8]`, no allocation.
#[macro_export]
macro_rules! get_bytes {
    () => {
        $crate::token().as_bytes()
    };
}

/// Buffered `print!`.
#[macro_export]
macro_rules! put {
    ($($arg:tt)*) => {
        unsafe {
            use ::std::io::Write;
            let _ = write!((*(&raw mut $crate::OUT)).as_mut().unwrap_unchecked(), $($arg)*);
        }
    };
}

/// Buffered `println!`.
#[macro_export]
macro_rules! putln {
    ($($arg:tt)*) => {
        unsafe {
            use ::std::io::Write;
            let _ = writeln!((*(&raw mut $crate::OUT)).as_mut().unwrap_unchecked(), $($arg)*);
        }
    };
}

// Template ends here
