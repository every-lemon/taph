//! Non-mutating specification-link, preview-status and source-inventory checks.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitCode};

fn main() -> ExitCode {
    let root = match repository_root() {
        Ok(root) => root,
        Err(error) => return fail(&error),
    };
    if let Err(error) = check_docs(&root) {
        return fail(&error);
    }
    println!("Specification links, examples and preview boundary verified");
    match check_inventory(&root) {
        Ok(count) => println!("Source inventory matches all {count} tracked files"),
        Err(error) => return fail(&error),
    }
    ExitCode::SUCCESS
}

fn fail(message: &str) -> ExitCode {
    eprintln!("error: {message}");
    ExitCode::FAILURE
}

fn repository_root() -> Result<PathBuf, String> {
    let out = Command::new("git")
        .args(["rev-parse", "--show-toplevel"])
        .output()
        .map_err(|e| format!("cannot run git: {e}"))?;
    if !out.status.success() {
        return Err("not inside a git repository".into());
    }
    let text = String::from_utf8(out.stdout).map_err(|_| "git path is not UTF-8".to_string())?;
    fs::canonicalize(text.trim()).map_err(|e| format!("cannot resolve repository root: {e}"))
}

fn tracked_files(root: &Path) -> Result<Vec<String>, String> {
    let out = Command::new("git")
        .args(["ls-files", "-z"])
        .current_dir(root)
        .output()
        .map_err(|e| format!("cannot run git ls-files: {e}"))?;
    if !out.status.success() {
        return Err("git ls-files failed".into());
    }
    let text =
        String::from_utf8(out.stdout).map_err(|_| "tracked path is not UTF-8".to_string())?;
    let mut files: Vec<String> = text
        .split('\0')
        .filter(|p| !p.is_empty())
        .map(String::from)
        .collect();
    files.sort();
    Ok(files)
}

fn check_docs(root: &Path) -> Result<(), String> {
    for relative in tracked_files(root)?.iter().filter(|p| p.ends_with(".md")) {
        let file = root.join(relative);
        let text = fs::read_to_string(&file).map_err(|e| format!("{relative}: {e}"))?;
        if text.lines().filter(|l| l.starts_with("```")).count() % 2 == 1 {
            return Err(format!("Unclosed example fence: {relative}"));
        }
        for link in local_links(&text) {
            let path_part = link.split('#').next().unwrap_or("");
            let target = file.parent().unwrap_or(root).join(path_part);
            let ok = fs::canonicalize(&target)
                .map(|t| t.starts_with(root))
                .unwrap_or(false);
            if !ok {
                return Err(format!(
                    "Invalid local documentation link: {relative}: {link}"
                ));
            }
        }
    }
    let readme =
        fs::read_to_string(root.join("README.md")).map_err(|e| format!("README.md: {e}"))?;
    if !readme.contains("no working compiler release yet") {
        return Err("README.md must state that there is no working compiler release yet".into());
    }
    let license = fs::read_to_string(root.join("LICENSE")).map_err(|e| format!("LICENSE: {e}"))?;
    if !license.contains("Apache License") {
        return Err("LICENSE must be the Apache License".into());
    }
    Ok(())
}

/// Targets of Markdown `](target)` links that point inside the repository.
fn local_links(text: &str) -> Vec<String> {
    let mut links = Vec::new();
    let mut rest = text;
    while let Some(start) = rest.find("](") {
        rest = &rest[start + 2..];
        let Some(end) = rest.find(')') else { break };
        let target = &rest[..end];
        let is_plain = !target.is_empty() && !target.chars().any(char::is_whitespace);
        let is_external = ["https:", "http:", "mailto:", "#"]
            .iter()
            .any(|p| target.starts_with(p));
        if is_plain && !is_external {
            links.push(target.to_string());
        }
        rest = &rest[end..];
    }
    links
}

fn check_inventory(root: &Path) -> Result<usize, String> {
    let raw = fs::read_to_string(root.join("packaging/source-files.json"))
        .map_err(|e| format!("packaging/source-files.json: {e}"))?;
    let entries = parse_string_array(&raw)?;
    let tracked = tracked_files(root)?;
    let mut sorted_unique = entries.clone();
    sorted_unique.sort();
    sorted_unique.dedup();
    if entries != sorted_unique || entries != tracked {
        let missing: Vec<&String> = tracked.iter().filter(|p| !entries.contains(p)).collect();
        let extra: Vec<&String> = entries.iter().filter(|p| !tracked.contains(p)).collect();
        return Err(format!(
            "Source inventory differs: missing={missing:?}, extra={extra:?}; require sorted unique paths"
        ));
    }
    Ok(entries.len())
}

/// Parses a JSON array of plain strings. Escapes are rejected so paths stay exact.
fn parse_string_array(raw: &str) -> Result<Vec<String>, String> {
    let body = raw.trim();
    let body = body
        .strip_prefix('[')
        .and_then(|b| b.strip_suffix(']'))
        .ok_or("Source inventory must be a list of paths")?;
    let mut entries = Vec::new();
    for item in body.split(',').map(str::trim).filter(|i| !i.is_empty()) {
        let value = item
            .strip_prefix('"')
            .and_then(|i| i.strip_suffix('"'))
            .ok_or("Source inventory must be a list of paths")?;
        if value.contains('\\') || value.contains('"') {
            return Err("Source inventory paths must not contain escapes".into());
        }
        entries.push(value.to_string());
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn finds_only_local_links() {
        let text = "[a](docs/A.md) [b](https://x.y) [c](#top) [d](mailto:a@b) [e](x y)";
        assert_eq!(local_links(text), vec!["docs/A.md".to_string()]);
    }

    #[test]
    fn parses_plain_string_arrays() {
        assert_eq!(
            parse_string_array("[\n \"a\",\n \"b\"\n]").unwrap(),
            vec!["a", "b"]
        );
        assert!(parse_string_array("{}").is_err());
        assert!(parse_string_array("[1]").is_err());
        assert!(parse_string_array("[\"a\\\\b\"]").is_err());
    }
}
