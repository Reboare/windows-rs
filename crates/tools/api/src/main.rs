use rayon::prelude::*;
use std::io::prelude::*;
use reader::*;
use quote::*;
use gen::*;

fn main() {
    let start = std::time::Instant::now();
    let mut output = std::path::PathBuf::from(workspace_dir());
    output.push("src/Windows");
    let _ = std::fs::remove_dir_all(&output);
    output.pop();

    let reader = TypeReader::get_mut();
    include_all(&mut reader.types);

    let root = reader.types.get_namespace("Windows").unwrap();

    let mut trees = Vec::new();
    collect_trees(&output, root.namespace, root, &mut trees);
    trees.par_iter().for_each(|tree| gen_tree(&output, root.namespace, tree));

    output.pop();
    output.push("Cargo.toml");

    write_toml(&output, root);
    println!("Elapsed: {} ms", start.elapsed().as_millis());
}

fn write_toml(output: &std::path::Path, tree: &TypeTree) {
    let mut file = std::fs::File::create(&output).unwrap();

    file.write_all(
        r#"
[package]
name = "windows"
version = "0.28.0"
authors = ["Microsoft"]
edition = "2018"
license = "MIT OR Apache-2.0"
description = "Rust for Windows"
repository = "https://github.com/microsoft/windows-rs"
documentation = "https://microsoft.github.io/windows-docs-rs/"
readme = ".github/readme.md"
exclude = [".github", ".windows", "docs", "tests"]

[workspace]
members = [
    "crates/deps/*",
    "crates/targets/*",
    "crates/tools/*",
    "crates/tests/legacy/*",
    "crates/tests/metadata/*",
    "crates/tests/winrt/*",
    "crates/tests/win32/*",
    "crates/tests/core",
    "crates/tests/sys",
]
exclude = ["crates/tests/component"]

[package.metadata.docs.rs]
default-target = "x86_64-pc-windows-msvc"
targets = []

[dependencies]
windows-sys = { path = "crates/deps/sys",  version = "0.28.0" }
windows_macros = { path = "crates/deps/macros",  version = "0.28.0", optional = true }
windows_reader = { path = "crates/deps/reader", version = "0.28.0", optional = true }
windows_gen = { path = "crates/deps/gen",  version = "0.28.0", optional = true }

[features]
default = []
std = []
alloc = []
deprecated = []
build = ["windows_gen", "windows_macros", "windows_reader"]
"#
        .as_bytes(),
    )
    .unwrap();

    write_features(&mut file, tree.namespace, tree);
}

fn write_features(file: &mut std::fs::File, root: &'static str, tree: &TypeTree) {
    for tree in tree.namespaces.values() {
        write_feature(file, root, tree);
        write_features(file, root, tree);
    }
}

fn write_feature(file: &mut std::fs::File, root: &'static str, tree: &TypeTree) {
    if !tree.include {
        return;
    }

    let feature = tree.namespace[root.len() + 1..].replace('.', "_");

    if let Some(pos) = feature.rfind('_') {
        let dependency = &feature[..pos];

        file.write_all(format!("{} = [\"{}\"]\n", feature, dependency).as_bytes()).unwrap();
    } else {
        file.write_all(format!("{} = []\n", feature).as_bytes()).unwrap();
    }
}

fn collect_trees<'a>(output: &std::path::Path, root: &'static str, tree: &'a TypeTree, trees: &mut Vec<&'a TypeTree>) {
    trees.push(tree);

    tree.namespaces.values().for_each(|tree| collect_trees(output, root, tree, trees));

    let mut path = std::path::PathBuf::from(output);
    path.push(tree.namespace.replace('.', "/"));
    std::fs::create_dir_all(&path).unwrap();
}

fn gen_tree(output: &std::path::Path, root: &'static str, tree: &TypeTree) {
    println!("{}", tree.namespace);
    let mut path = std::path::PathBuf::from(output);

    path.push(tree.namespace.replace('.', "/"));
    path.push("mod.rs");

    let tokens = gen_source_file(root, tree, false);

    let mut child = std::process::Command::new("rustfmt").stdin(std::process::Stdio::piped()).stdout(std::process::Stdio::piped()).spawn().expect("Failed to spawn `rustfmt`");
    let mut stdin = child.stdin.take().expect("Failed to open stdin");
    stdin.write_all(tokens.into_string().as_bytes()).unwrap();
    drop(stdin);

    let output = child.wait_with_output().unwrap();
    assert!(output.status.success());
    std::fs::write(&path, String::from_utf8(output.stdout).expect("Failed to parse UTF-8")).unwrap();
}

fn gen_source_file(root: &'static str, tree: &TypeTree, ignore_windows_features: bool) -> TokenStream {
    let gen = Gen { relative: tree.namespace, root, ignore_windows_features, docs: false, build: false };

    let types = tree.types.values().map(move |t| gen_type_entry(t, &gen));

    let namespaces = tree.namespaces.iter().filter_map(move |(name, tree)| {
        if !tree.include {
            return None;
        }

        let name = to_ident(name);
        let namespace = tree.namespace[tree.namespace.find('.').unwrap() + 1..].replace('.', "_");
        Some(quote! {
            #[cfg(feature = #namespace)] pub mod #name;
        })
    });

    quote! {
        #![allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
        #(#namespaces)*
        #(#types)*
    }
}
