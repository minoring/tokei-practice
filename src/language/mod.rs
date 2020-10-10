use std::collections::BTreeMap;

// A struct representing statistics about a single Language.
#[derive(Clone, Debug, Deserialize, Default, PartialEq, Serialize)]
pub struct Language {
    /// The total number of blank lines.
    pub blanks: usize,
    /// The total number of lines of code.
    pub code: usize,
    /// The total number of comments(both single, and multi-line)
    pub comments: usize,
    /// A collection of statistics of individual files.
    pub reports: Vec<Report>,
    /// A map of any languages found in the reports.
    pub children: BTreeMap<LanguageType, Vec<Report>>,
    /// Whether this language had problems with file parsing
    pub inaccurate: bool,
}
