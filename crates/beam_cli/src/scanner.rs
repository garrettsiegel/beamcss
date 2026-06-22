use std::{
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
};

const SUPPORTED_EXTENSIONS: &[&str] = &["html", "jsx", "tsx", "vue", "svelte", "astro"];
const IGNORED_DIRS: &[&str] = &[
    ".git",
    "node_modules",
    "dist",
    "target",
    ".vite",
    ".next",
    "coverage",
];

pub(crate) fn collect_files(paths: &[PathBuf]) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for path in paths {
        collect_path(path, &mut files)?;
    }
    files.sort();
    files.dedup();
    Ok(files)
}

fn collect_path(path: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    if path.is_file() {
        if is_supported_source(path) {
            files.push(path.to_owned());
        }
        return Ok(());
    }

    if !path.is_dir() {
        eprintln!("[beam] content path not found: {}", path.display());
        return Ok(());
    }

    if ignored_dir(path) {
        return Ok(());
    }

    for entry in fs::read_dir(path)? {
        collect_path(&entry?.path(), files)?;
    }
    Ok(())
}

fn ignored_dir(path: &Path) -> bool {
    path.file_name()
        .and_then(OsStr::to_str)
        .is_some_and(|name| IGNORED_DIRS.contains(&name))
}

fn is_supported_source(path: &Path) -> bool {
    path.extension()
        .and_then(OsStr::to_str)
        .is_some_and(|extension| SUPPORTED_EXTENSIONS.contains(&extension))
}

pub(crate) fn scan_files(files: &[PathBuf]) -> io::Result<Vec<String>> {
    let mut classes = Vec::new();
    for file in files {
        let source = fs::read_to_string(file)?;
        classes.extend(extract_class_strings(&source));
    }
    Ok(classes)
}

pub(crate) fn extract_class_strings(source: &str) -> Vec<String> {
    let mut classes = Vec::new();
    for attribute in ["class", "className"] {
        let mut offset = 0usize;
        while let Some(index) = source[offset..].find(attribute) {
            let start = offset + index;
            if !attribute_boundary(source, start, attribute.len()) {
                offset = start + attribute.len();
                continue;
            }
            if let Some((class_string, next)) =
                read_attribute_value(source, start + attribute.len())
            {
                classes.push(class_string);
                offset = next;
            } else {
                offset = start + attribute.len();
            }
        }
    }
    classes
}

fn attribute_boundary(source: &str, start: usize, len: usize) -> bool {
    let before = source[..start].chars().next_back();
    let after = source[start + len..].chars().next();
    !before.is_some_and(is_ident_char) && !after.is_some_and(is_ident_char)
}

fn is_ident_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == '-'
}

fn read_attribute_value(source: &str, mut index: usize) -> Option<(String, usize)> {
    index = skip_ws(source, index);
    if source[index..].chars().next()? != '=' {
        return None;
    }
    index += 1;
    index = skip_ws(source, index);

    let close_brace = if source[index..].starts_with('{') {
        index += 1;
        index = skip_ws(source, index);
        true
    } else {
        false
    };

    let quote = source[index..].chars().next()?;
    if quote != '"' && quote != '\'' && quote != '`' {
        return None;
    }
    index += quote.len_utf8();
    let value_start = index;
    let mut escaped = false;

    while index < source.len() {
        let ch = source[index..].chars().next()?;
        if escaped {
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == quote {
            let value = source[value_start..index].to_owned();
            index += ch.len_utf8();
            if close_brace {
                index = skip_ws(source, index);
                if source[index..].starts_with('}') {
                    index += 1;
                }
            }
            return Some((value, index));
        }
        index += ch.len_utf8();
    }

    None
}

fn skip_ws(source: &str, mut index: usize) -> usize {
    while index < source.len() {
        let Some(ch) = source[index..].chars().next() else {
            break;
        };
        if !ch.is_whitespace() {
            break;
        }
        index += ch.len_utf8();
    }
    index
}
