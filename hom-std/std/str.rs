// ============================================================
// Homun String Library — included by std.rs
// ============================================================

pub fn split(s: impl AsRef<str>, sep: impl AsRef<str>) -> Vec<String> {
    s.as_ref().split(sep.as_ref()).map(|x| x.to_string()).collect()
}

pub fn join(v: impl AsRef<[String]>, sep: impl AsRef<str>) -> String {
    v.as_ref().join(sep.as_ref())
}

pub fn trim(s: impl AsRef<str>) -> String { s.as_ref().trim().to_string() }
pub fn trim_start(s: impl AsRef<str>) -> String { s.as_ref().trim_start().to_string() }
pub fn trim_end(s: impl AsRef<str>) -> String { s.as_ref().trim_end().to_string() }

pub fn starts_with(s: impl AsRef<str>, prefix: impl AsRef<str>) -> bool { s.as_ref().starts_with(prefix.as_ref()) }
pub fn ends_with(s: impl AsRef<str>, suffix: impl AsRef<str>) -> bool { s.as_ref().ends_with(suffix.as_ref()) }

pub fn replace(s: impl AsRef<str>, from: impl AsRef<str>, to: impl AsRef<str>) -> String { s.as_ref().replace(from.as_ref(), to.as_ref()) }

pub fn to_upper(s: impl AsRef<str>) -> String { s.as_ref().to_uppercase() }
pub fn to_lower(s: impl AsRef<str>) -> String { s.as_ref().to_lowercase() }

pub fn chars(s: impl AsRef<str>) -> Vec<String> {
    s.as_ref().chars().map(|c| c.to_string()).collect()
}

pub fn find(s: impl AsRef<str>, sub: impl AsRef<str>) -> i32 {
    s.as_ref().find(sub.as_ref()).map(|i| i as i32).unwrap_or(-1)
}

pub fn contains(s: impl AsRef<str>, sub: impl AsRef<str>) -> bool { s.as_ref().contains(sub.as_ref()) }

pub fn repeat(s: impl AsRef<str>, n: i32) -> String { s.as_ref().repeat(n as usize) }

pub fn substr(s: impl AsRef<str>, start: i32, end: i32) -> String {
    let s = s.as_ref();
    let len = s.len() as i32;
    let s_ = if start < 0 { (len + start).max(0) } else { start.min(len) } as usize;
    let e_ = if end < 0 { (len + end).max(0) } else { end.min(len) } as usize;
    s.get(s_..e_).unwrap_or("").to_string()
}

pub fn strip_prefix(s: impl AsRef<str>, prefix: impl AsRef<str>) -> String {
    let s = s.as_ref();
    s.strip_prefix(prefix.as_ref()).unwrap_or(s).to_string()
}

pub fn strip_suffix(s: impl AsRef<str>, suffix: impl AsRef<str>) -> String {
    let s = s.as_ref();
    s.strip_suffix(suffix.as_ref()).unwrap_or(s).to_string()
}

pub fn lines(s: impl AsRef<str>) -> Vec<String> {
    s.as_ref().lines().map(|l| l.to_string()).collect()
}

pub fn is_empty(s: impl AsRef<str>) -> bool { s.as_ref().is_empty() }

pub fn is_digit(s: impl AsRef<str>) -> bool { s.as_ref().chars().all(|c| c.is_ascii_digit()) }
pub fn is_alpha(s: impl AsRef<str>) -> bool { s.as_ref().chars().all(|c| c.is_alphabetic()) }
pub fn is_alnum(s: impl AsRef<str>) -> bool { s.as_ref().chars().all(|c| c.is_alphanumeric()) }
pub fn is_upper(s: impl AsRef<str>) -> bool { s.as_ref().chars().all(|c| c.is_uppercase()) }
pub fn is_lower(s: impl AsRef<str>) -> bool { s.as_ref().chars().all(|c| c.is_lowercase()) }

pub fn pad_left(s: impl AsRef<str>, width: i32, fill: impl AsRef<str>) -> String {
    let s = s.as_ref();
    let fill = fill.as_ref();
    let w = width as usize;
    if s.len() >= w { return s.to_string(); }
    let pad = fill.repeat((w - s.len()) / fill.len().max(1) + 1);
    format!("{}{}", &pad[..w - s.len()], s)
}

pub fn pad_right(s: impl AsRef<str>, width: i32, fill: impl AsRef<str>) -> String {
    let s = s.as_ref();
    let fill = fill.as_ref();
    let w = width as usize;
    if s.len() >= w { return s.to_string(); }
    let pad = fill.repeat((w - s.len()) / fill.len().max(1) + 1);
    format!("{}{}", s, &pad[..w - s.len()])
}

pub fn char_at(s: impl AsRef<str>, i: i32) -> String {
    let s = s.as_ref();
    let len = s.chars().count() as i32;
    let idx = if i < 0 { (len + i).max(0) } else { i } as usize;
    s.chars().nth(idx).map(|c| c.to_string()).unwrap_or_default()
}

pub fn parse_int(s: impl AsRef<str>) -> i32 {
    s.as_ref().trim().parse::<i32>().unwrap_or(0)
}

pub fn parse_float(s: impl AsRef<str>) -> f32 {
    s.as_ref().trim().parse::<f32>().unwrap_or(0.0)
}
