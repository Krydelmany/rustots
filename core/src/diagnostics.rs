use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub level: DiagnosticLevel,
    pub message: String,
    pub location: DiagnosticLocation,
    pub code: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiagnosticLevel {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagnosticLocation {
    pub line: usize,
    pub column: usize,
    pub length: usize,
}

pub struct DiagnosticCollector {
    diagnostics: Vec<Diagnostic>,
}

impl DiagnosticCollector {
    pub fn new() -> Self {
        Self {
            diagnostics: Vec::new(),
        }
    }

    pub fn error(&mut self, message: String, location: DiagnosticLocation, code: Option<String>) {
        self.diagnostics.push(Diagnostic {
            level: DiagnosticLevel::Error,
            message,
            location,
            code,
        });
    }

    pub fn warning(&mut self, message: String, location: DiagnosticLocation, code: Option<String>) {
        self.diagnostics.push(Diagnostic {
            level: DiagnosticLevel::Warning,
            message,
            location,
            code,
        });
    }

    pub fn info(&mut self, message: String, location: DiagnosticLocation, code: Option<String>) {
        self.diagnostics.push(Diagnostic {
            level: DiagnosticLevel::Info,
            message,
            location,
            code,
        });
    }

    pub fn get_diagnostics(&self) -> &Vec<Diagnostic> {
        &self.diagnostics
    }

    pub fn has_errors(&self) -> bool {
        self.diagnostics.iter().any(|d| matches!(d.level, DiagnosticLevel::Error))
    }

    pub fn has_warnings(&self) -> bool {
        self.diagnostics.iter().any(|d| matches!(d.level, DiagnosticLevel::Warning))
    }

    pub fn clear(&mut self) {
        self.diagnostics.clear();
    }

    pub fn extend(&mut self, other: &DiagnosticCollector) {
        self.diagnostics.extend(other.diagnostics.clone());
    }

    pub fn error_count(&self) -> usize {
        self.diagnostics.iter()
            .filter(|d| matches!(d.level, DiagnosticLevel::Error))
            .count()
    }

    pub fn warning_count(&self) -> usize {
        self.diagnostics.iter()
            .filter(|d| matches!(d.level, DiagnosticLevel::Warning))
            .count()
    }
}

impl Default for DiagnosticCollector {
    fn default() -> Self {
        Self::new()
    }
}
