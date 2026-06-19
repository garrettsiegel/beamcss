use beam_core::{CompileMessage, ExplainResult};

pub(crate) fn check_text(class_string_count: usize, errors: &[CompileMessage]) -> String {
    if errors.is_empty() {
        return format!("Beam check passed: {class_string_count} class string(s) valid.\n");
    }

    let mut output = format!("Beam check failed: {} error(s).\n", errors.len());
    for error in errors {
        output.push_str(&format!("- {}: {}\n", error.class_name, error.message));
    }
    output
}

pub(crate) fn explain_text(result: &ExplainResult) -> String {
    let mut output = String::new();
    for class_string in &result.class_strings {
        output.push_str(&format!("Class string: {}\n", class_string.class_string));
        for token in &class_string.tokens {
            output.push_str(&format!("- {} ({})\n", token.raw, token.kind));
            for atom in &token.atoms {
                let media = if atom.media.is_empty() {
                    String::new()
                } else {
                    format!(" @media {}", atom.media.join(" and "))
                };
                output.push_str(&format!(
                    "  -> {}{} [{}] {}",
                    atom.selector, media, atom.layer, atom.declaration
                ));
                output.push('\n');
            }
            for error in &token.errors {
                output.push_str(&format!("  ! {}: {}\n", error.class_name, error.message));
            }
        }
    }

    if !result.errors.is_empty() {
        output.push_str(&format!("Errors: {}\n", result.errors.len()));
    }
    output
}
