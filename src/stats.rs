use std::{path::PathBuf};


pub struct Report {
  /// The code statistics found in the file.
  pub stats: CodeStats,
  /// File name.
  pub name: PathBuf,
}
