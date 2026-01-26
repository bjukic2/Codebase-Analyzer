mod analysis;
mod cli;
mod parser;
mod walker;

use parser::typescript::TsParser;
use std::fs;

fn print_tree(node: tree_sitter::Node, indent: usize) {
    let indent_str = " ".repeat(indent);
    println!("{}{}", indent_str, node.kind());

    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            print_tree(child, indent + 2);
        }
    }
}

fn main() {
    let mut ts = TsParser::new();

    let code = fs::read_to_string("./test.ts").unwrap();
    let tree = ts.parse(&code).unwrap();

    let root = tree.root_node();

    let functions = analysis::extract_functions(root, &code);

    println!("Functions found:");
    for f in functions {
        println!(
            "Function name: {} (lines {}-{}, total {})",
            f.name,
            f.start_line + 1,
            f.end_line + 1,
            f.line_count
        );
    }
}
