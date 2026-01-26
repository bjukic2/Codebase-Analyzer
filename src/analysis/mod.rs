use tree_sitter::Node;

#[derive(Debug)]
pub struct FunctionInfo {
    pub name: String,
    pub start_line: usize,
    pub end_line: usize,
    pub line_count: usize,
}

pub fn extract_functions(root: Node, source: &str) -> Vec<FunctionInfo> {
    let mut functions = Vec::new();
    walk(root, source, &mut functions);
    functions
}

fn walk(node: Node, source: &str, out: &mut Vec<FunctionInfo>) {
    match node.kind() {
        "function_declaration" | "method_definition" | "arrow_function" => {
            if let Some(info) = extract_function_info(node, source) {
                out.push(info);
            }
        }
        _ => {}
    }
    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            walk(child, source, out);
        }
    }
}

fn extract_function_info(node: Node, source: &str) -> Option<FunctionInfo> {
    let mut name = "<anonymous>".to_string();

    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            match child.kind() {
                "identifier" | "property_identifier" => {
                    name = child.utf8_text(source.as_bytes()).ok()?.to_string();
                }
                _ => {}
            }
        }
    }

    if name == "<anonymous>" {
        if let Some(id) = find_variable_name(node, source) {
            name = id;
        }
    }

    let start = node.start_position().row;
    let end = node.end_position().row;
    let count = end - start;

    Some(FunctionInfo {
        name,
        start_line: start,
        end_line: end,
        line_count: count,
    })
}

fn find_variable_name(node: Node, source: &str) -> Option<String> {
    let parent = node.parent()?;
    if parent.kind() == "variable_declarator" {
        for i in 0..parent.child_count() {
            if let Some(child) = parent.child(i) {
                if child.kind() == "identifier" {
                    return Some(child.utf8_text(source.as_bytes()).ok()?.to_string());
                }
            }
        }
    }
    None
}
