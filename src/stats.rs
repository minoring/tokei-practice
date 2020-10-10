use std::{collections::BTreeMap, path::PathBuf};

use crate::LanguageType;


/// A struct representing stats about a single blob of code.
#[derive(Clone, Debug, Default, PartialEq)]
#[non_exhaustive]
pub struct CodeStats {
  /// The blan lines in the blob.
  pub blanks: usize,
  /// The lines of code in the blob.
  pub code: usize,
  /// The lines of comments in the blob.
  pub comments: usize,
  /// Language blobs that were contained inside this blob.
  pub blobs: BTreeMap<LanguageType, CodeStats>,
}

pub struct Report {
  /// The code statistics found in the file.
  pub stats: CodeStats,
  /// File name.
  pub name: PathBuf,
}
