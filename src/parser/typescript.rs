use tree_sitter::{Parser, Tree};
use tree_sitter_typescript::language_tsx;

pub struct TsParser {
    parser: Parser,
}

impl TsParser {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(language_tsx())
            .expect("Failed to load TypeScript grammar!");

        Self { parser }
    }

    pub fn parse(&mut self, source: &str) -> Option<Tree> {
        self.parser.parse(source, None)
    }
}
