use std::fs;

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/main.rs");

    let name = std::env::var("CARGO_PKG_NAME").unwrap().replace('-', "_");
    let prefix = format!("use {name}::");

    let lib: String = fs::read_to_string("src/lib.rs")
        .unwrap()
        .lines()
        .map(|l| l.strip_prefix("pub ").unwrap_or(l))
        .collect::<Vec<_>>()
        .join("\n");
    let main: String = fs::read_to_string("src/main.rs")
        .unwrap()
        .lines()
        .filter(|l| !l.trim_start().starts_with(&prefix))
        .collect::<Vec<_>>()
        .join("\n");

    fs::write("send.rs", format!("{lib}\n\n{main}\n")).unwrap();
}
