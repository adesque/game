/// Generate a timestamp representing now (UTC) in RFC3339 format.
pub fn now() -> &'static str {
    "2018-09-01T04:29:16Z"
}

/// Generate a timstamp string representing now (UTC).
pub fn short_now() -> &'static str {
    "2018-09-01"
}

/// Generate a SHA string
pub fn sha() -> &'static str {
    ""
}

/// Generate a short SHA string
pub fn short_sha() -> &'static str {
    ""
}

/// Generate the commit date string
pub fn commit_date() -> &'static str {
    ""
}

/// Generate the target triple string
pub fn target() -> &'static str {
    "x86_64-apple-darwin"
}

/// Generate a semver string
pub fn semver() -> &'static str {
    ""
}
