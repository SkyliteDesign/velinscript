// Formatting Configuration

#[derive(Debug, Clone)]
pub struct FormatConfig {
    pub indent_size: usize,
    pub indent_style: IndentStyle,
    pub line_width: usize,
    pub tab_width: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IndentStyle {
    Spaces,
    Tabs,
}

impl Default for FormatConfig {
    fn default() -> Self {
        FormatConfig {
            indent_size: 4,
            indent_style: IndentStyle::Spaces,
            line_width: 100,
            tab_width: 4,
        }
    }
}

impl FormatConfig {
    pub fn new() -> Self {
        Self::default()
    }
    
    pub fn indent_string(&self) -> String {
        match self.indent_style {
            IndentStyle::Spaces => " ".repeat(self.indent_size),
            IndentStyle::Tabs => "\t".repeat(self.tab_width),
        }
    }
}
