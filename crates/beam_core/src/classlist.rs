pub(crate) fn split_classlist(classlist: &str) -> Vec<&str> {
    let mut tokens = Vec::new();
    let mut start = None;
    let mut bracket_depth = 0usize;
    let mut paren_depth = 0usize;

    for (index, ch) in classlist.char_indices() {
        if start.is_none() && !ch.is_whitespace() {
            start = Some(index);
        }

        match ch {
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '(' if bracket_depth == 0 => paren_depth += 1,
            ')' if bracket_depth == 0 => paren_depth = paren_depth.saturating_sub(1),
            _ => {}
        }

        if ch.is_whitespace() && bracket_depth == 0 && paren_depth == 0 {
            if let Some(token_start) = start.take() {
                tokens.push(&classlist[token_start..index]);
            }
        }
    }

    if let Some(token_start) = start {
        tokens.push(&classlist[token_start..]);
    }

    tokens
}
