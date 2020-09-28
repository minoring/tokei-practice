use clap::ArgMatches;
use tokei_practice::Sort;

#[derive(Debug)]
pub struct Cli<'a> {
    matches: ArgMatches<'a>,
    pub columns: Option<usize>,
    pub files: bool,
    pub hidden: bool,
    pub no_ignore: bool,
    pub no_ignore_parent: bool,
    pub no_ignore_dot: bool,
    pub no_ignore_vcs: bool,
    pub output: Option<Format>,
    pub print_languages: bool,
    pub sort: Option<Sort>,
    pub types: Option<Vec<LanguageType>>,
    pub number_format: num_format::CustomFormat,
    pub verbose: u64,
}

impl<'a> Cli<'a> {}
