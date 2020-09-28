#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Sort {
    /// Sort by number blank lines.
    Blanks,
    /// Sort by number comment lines.
    Comments,
    /// Sort by number files lines.
    Files,
    /// Sort by number of lines.
    Lines,
}
