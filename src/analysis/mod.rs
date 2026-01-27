use tree_sitter::Node;

#[derive(Debug)]
pub struct FunctionInfo {
    pub name: String,
    pub start_line: usize,
    pub end_line: usize,
    pub line_count: usize,
    pub complexity: usize,
    pub contributions: Vec<ComplexityContribution>,
}

#[derive(Debug)]
pub struct ComplexityContribution {
    pub line: usize,
    pub kind: String,
    pub description: String,
}

pub fn extract_functions(root: Node, source: &str) -> Vec<FunctionInfo> {
    let mut functions = Vec::new();
    walk(root, source, &mut functions);
    functions
}

fn walk(node: Node, source: &str, out: &mut Vec<FunctionInfo>) {
    let is_function = matches!(
        node.kind(),
        "function_declaration" | "method_definition" | "arrow_function" | "function_expression"
    );

    if is_function {
        if let Some(info) = extract_function_info(node, source) {
            out.push(info);
        }
    }

    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            walk(child, source, out);
        }
    }
}

fn extract_function_info(node: Node, source: &str) -> Option<FunctionInfo> {
    let mut name = "<anonymous>".to_string();

    match node.kind() {
        "function_declaration" => {
            if let Some(id) = node.child_by_field_name("name") {
                name = id.utf8_text(source.as_bytes()).ok()?.to_string();
            }
        }

        "method_definition" => {
            if let Some(id) = node.child_by_field_name("name") {
                name = id.utf8_text(source.as_bytes()).ok()?.to_string();
            }
        }

        "arrow_function" | "function_expression" => {
            if let Some(id) = find_variable_name(node, source) {
                name = id;
            } else if let Some(id) = find_object_key(node, source) {
                name = id;
            }
        }

        _ => {}
    }

    let body = find_body(node)?;
    let (complexity, contributions) = calculate_complexity(body, source);
    let start = body.start_position().row;
    let end = body.end_position().row;
    let count = end - start;

    Some(FunctionInfo {
        name,
        start_line: start,
        end_line: end,
        line_count: count,
        complexity,
        contributions,
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

fn find_object_key(node: Node, source: &str) -> Option<String> {
    let parent = node.parent()?;
    if parent.kind() == "pair" {
        if let Some(key) = parent.child_by_field_name("key") {
            return Some(key.utf8_text(source.as_bytes()).ok()?.to_string());
        }
    }
    None
}

fn find_body(node: Node) -> Option<Node> {
    if let Some(body) = node.child_by_field_name("body") {
        return Some(body);
    }

    for i in 0..node.child_count() {
        if let Some(child) = node.child(i) {
            if child.kind() == "statement_block" {
                return Some(child);
            }
        }
    }

    None
}

pub fn calculate_complexity(body: Node, source: &str) -> (usize, Vec<ComplexityContribution>) {
    let mut contributions = Vec::new();

    fn walk(node: Node, source: &str, out: &mut Vec<ComplexityContribution>) {
        let kind = node.kind();

        if kind == "if_statement" {
            out.push(ComplexityContribution {
                line: node.start_position().row,
                kind: "if".into(),
                description: "Branch introduced by if-statement.".into(),
            });
        }

        if kind == "for_statement" {
            out.push(ComplexityContribution {
                line: node.start_position().row,
                kind: "for".into(),
                description: "Branch introduced by for-statement.".into(),
            });
        }

        if kind == "while_statement" {
            out.push(ComplexityContribution {
                line: node.start_position().row,
                kind: "while".into(),
                description: "Branch introduced by while-statement.".into(),
            });
        }

        if kind == "do_statement" {
            out.push(ComplexityContribution {
                line: node.start_position().row,
                kind: "do-while".into(),
                description: "Branch introduced by do-while-statement.".into(),
            });
        }

        if kind == "switch_statement" {
            out.push(ComplexityContribution {
                line: node.start_position().row,
                kind: "switch".into(),
                description: "Branch introduced by switch-statement.".into(),
            });
        }

        if kind == "switch_case" {
            out.push(ComplexityContribution {
                line: node.start_position().row,
                kind: "case".into(),
                description: "Branch introduced by case.".into(),
            });
        }

        if kind == "conditional_expression" {
            out.push(ComplexityContribution {
                line: node.start_position().row,
                kind: "ternary".into(),
                description: "Branch introduced by ternary operator.".into(),
            });
        }

        if kind == "binary_expression" {
            if let Some(op) = node.child_by_field_name("operator") {
                if let Ok(text) = op.utf8_text(source.as_bytes()) {
                    if text == "&&" || text == "||" {
                        out.push(ComplexityContribution {
                            line: node.start_position().row,
                            kind: text.into(),
                            description: format!("Logical operator '{}' introduces a branch", text),
                        });
                    }
                }
            }
        }

        for i in 0..node.child_count() {
            if let Some(child) = node.child(i) {
                walk(child, source, out);
            }
        }
    }

    walk(body, source, &mut contributions);

    let complexity = 1 + contributions.len();

    (complexity, contributions)
}
