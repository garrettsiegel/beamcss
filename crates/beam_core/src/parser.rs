use crate::{classlist::split_classlist, utility_groups, ClassToken, CompileMessage};

pub fn parse_classlist(classlist: &str) -> Result<Vec<ClassToken>, Vec<CompileMessage>> {
    let mut tokens = Vec::new();
    let mut errors = Vec::new();

    for raw in split_classlist(classlist) {
        match parse_token(raw) {
            Ok(token) => tokens.push(token),
            Err(message) => errors.push(CompileMessage {
                class_name: raw.to_owned(),
                message,
            }),
        }
    }

    if errors.is_empty() {
        Ok(tokens)
    } else {
        Err(errors)
    }
}

pub fn parse_flat_classlist(classlist: &str) -> Vec<&str> {
    classlist.split_whitespace().collect()
}

pub(crate) fn parse_token(raw: &str) -> Result<ClassToken, String> {
    if raw.is_empty() {
        return Err("empty class token".to_owned());
    }

    let (variants, base) = parse_utility(raw)?;
    if let Some(open) = top_level_group_open(raw)? {
        let close = raw
            .len()
            .checked_sub(1)
            .filter(|_| raw.ends_with(')'))
            .ok_or_else(|| "group is missing a closing `)`".to_owned())?;
        let head = &raw[..open];
        if head.ends_with('-') {
            return Ok(ClassToken::Utility { variants, base });
        }
        let inner = &raw[open + 1..close];
        if inner.trim().is_empty() {
            return Err("group has no children".to_owned());
        }
        if let Some((head_variants, head_name)) = parse_utility_group_head(head)? {
            let children = utility_groups::parse_utility_group(&head_name, inner)?;
            return Ok(ClassToken::Group {
                variants: head_variants,
                children,
            });
        }

        let (head_variants, head_name) = parse_head(head)?;
        let children = parse_classlist(inner).map_err(|errors| {
            errors
                .into_iter()
                .map(|error| format!("{}: {}", error.class_name, error.message))
                .collect::<Vec<_>>()
                .join("; ")
        })?;

        if let Some(name) = head_name {
            return Err(format!("unrecognized group syntax `{name}(...)`"));
        }

        if head_variants.is_empty() {
            return Err("group is missing a variant chain".to_owned());
        }

        return Ok(ClassToken::Group {
            variants: head_variants,
            children,
        });
    }

    Ok(ClassToken::Utility { variants, base })
}

fn parse_utility_group_head(head: &str) -> Result<Option<(Vec<String>, String)>, String> {
    let Some(group_head) = head.strip_suffix(':') else {
        return Ok(None);
    };

    let mut parts = split_variant_chain(group_head)?;
    let Some(name) = parts.pop() else {
        return Ok(None);
    };

    if utility_groups::is_utility_group(&name) {
        return Ok(Some((parts, name)));
    }

    Ok(None)
}

fn top_level_group_open(raw: &str) -> Result<Option<usize>, String> {
    let mut bracket_depth = 0usize;
    let mut paren_depth = 0usize;
    let mut first_open = None;

    for (index, ch) in raw.char_indices() {
        match ch {
            '[' if paren_depth == 0 => bracket_depth += 1,
            ']' if paren_depth == 0 => {
                bracket_depth = bracket_depth
                    .checked_sub(1)
                    .ok_or_else(|| "unmatched `]` in class token".to_owned())?;
            }
            '(' if bracket_depth == 0 => {
                if paren_depth == 0 {
                    first_open = Some(index);
                }
                paren_depth += 1;
            }
            ')' if bracket_depth == 0 => {
                paren_depth = paren_depth
                    .checked_sub(1)
                    .ok_or_else(|| "unmatched `)` in class token".to_owned())?;
            }
            _ => {}
        }
    }

    if bracket_depth != 0 {
        return Err("unclosed `[` in class token".to_owned());
    }
    if paren_depth != 0 {
        return Err("unclosed `(` in class token".to_owned());
    }

    Ok(first_open)
}

fn parse_head(head: &str) -> Result<(Vec<String>, Option<String>), String> {
    if head.is_empty() {
        return Err("group is missing a head".to_owned());
    }

    if let Some(variant_head) = head.strip_suffix(':') {
        return Ok((split_variant_chain(variant_head)?, None));
    }

    let mut parts = split_variant_chain(head)?;
    let name = parts
        .pop()
        .ok_or_else(|| "group is missing a name".to_owned())?;
    Ok((parts, Some(name)))
}

fn parse_utility(raw: &str) -> Result<(Vec<String>, String), String> {
    let mut split_at = None;
    let mut bracket_depth = 0usize;
    let mut paren_depth = 0usize;

    for (index, ch) in raw.char_indices() {
        match ch {
            '[' => bracket_depth += 1,
            ']' => {
                bracket_depth = bracket_depth
                    .checked_sub(1)
                    .ok_or_else(|| "unmatched `]` in utility".to_owned())?;
            }
            '(' => paren_depth += 1,
            ')' => {
                paren_depth = paren_depth
                    .checked_sub(1)
                    .ok_or_else(|| "unmatched `)` in utility".to_owned())?;
            }
            ':' if bracket_depth == 0 && paren_depth == 0 => split_at = Some(index),
            _ => {}
        }
    }

    if bracket_depth != 0 {
        return Err("unclosed `[` in utility".to_owned());
    }
    if paren_depth != 0 {
        return Err("unclosed `(` in utility".to_owned());
    }

    if let Some(index) = split_at {
        let variants = split_variant_chain(&raw[..index])?;
        let base = raw[index + 1..].to_owned();
        if base.is_empty() {
            return Err("utility is missing a base".to_owned());
        }
        Ok((variants, base))
    } else {
        Ok((Vec::new(), raw.to_owned()))
    }
}

fn split_variant_chain(raw: &str) -> Result<Vec<String>, String> {
    if raw.is_empty() {
        return Ok(Vec::new());
    }

    let mut variants = Vec::new();
    let mut start = 0usize;
    let mut bracket_depth = 0usize;

    for (index, ch) in raw.char_indices() {
        match ch {
            '[' => bracket_depth += 1,
            ']' => {
                bracket_depth = bracket_depth
                    .checked_sub(1)
                    .ok_or_else(|| "unmatched `]` in variant chain".to_owned())?;
            }
            ':' if bracket_depth == 0 => {
                let part = &raw[start..index];
                if part.is_empty() {
                    return Err("empty variant in variant chain".to_owned());
                }
                variants.push(part.to_owned());
                start = index + 1;
            }
            _ => {}
        }
    }

    if bracket_depth != 0 {
        return Err("unclosed `[` in variant chain".to_owned());
    }

    let part = &raw[start..];
    if part.is_empty() {
        return Err("empty variant in variant chain".to_owned());
    }
    variants.push(part.to_owned());
    Ok(variants)
}
