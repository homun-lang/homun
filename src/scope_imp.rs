pub type Scope = Vec<String>;

// scope_from_list needs a generic (<S: Into<String>>) to accept both
// Vec<String> and Vec<&str> (string literals in Homun list literals
// desugar to &str in the generated vec![] macro).
pub fn scope_from_list<S: Into<String>>(list: Vec<S>) -> Scope {
    list.into_iter().map(|s| s.into()).collect()
}
