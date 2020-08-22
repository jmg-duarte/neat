use glob::{glob_with, MatchOptions};

fn get_match_options(case_sensitive: bool) -> MatchOptions {
    let mut match_opts = MatchOptions::new();
    match_opts.case_sensitive = case_sensitive;
    match_opts
}

fn build_glob(folder: &str, glob: &str) -> String {
    let mut result = folder.to_owned();
    if !result.ends_with("/") && !glob.starts_with("/") {
        result.push('/');
    }
    result.push_str(glob);
    result
}