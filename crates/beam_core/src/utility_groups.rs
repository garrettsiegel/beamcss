use crate::{classlist::split_classlist, ClassToken};

pub(crate) fn is_utility_group(name: &str) -> bool {
    matches!(name, "padding" | "margin" | "text" | "border")
}

pub(crate) fn parse_utility_group(name: &str, inner: &str) -> Result<Vec<ClassToken>, String> {
    split_classlist(inner)
        .into_iter()
        .map(|child| utility_group_child(name, child))
        .collect()
}

fn utility_group_child(name: &str, child: &str) -> Result<ClassToken, String> {
    let base = match name {
        "padding" => spacing_child("p", child)?,
        "margin" => spacing_child("m", child)?,
        "text" => text_child(child)?,
        "border" => border_child(child)?,
        _ => return Err(format!("unsupported utility group `{name}`")),
    };

    Ok(ClassToken::Utility {
        variants: Vec::new(),
        base,
    })
}

fn spacing_child(prefix: &str, child: &str) -> Result<String, String> {
    let Some((side, value)) = split_child_pair(child) else {
        return Ok(format!("{prefix}-{child}"));
    };

    let axis = match side {
        "x" => "x",
        "y" => "y",
        "top" | "t" => "t",
        "right" | "r" => "r",
        "bottom" | "b" => "b",
        "left" | "l" => "l",
        _ => return Err(format!("unsupported {prefix} group key `{side}`")),
    };

    Ok(format!("{prefix}{axis}-{value}"))
}

fn text_child(child: &str) -> Result<String, String> {
    let Some((key, value)) = split_child_pair(child) else {
        return Ok(match child {
            "left" | "center" | "right" => format!("text-{child}"),
            "thin" | "extralight" | "light" | "normal" | "medium" | "semibold" | "bold"
            | "extrabold" | "black" => format!("font-{child}"),
            _ => format!("text-{child}"),
        });
    };

    match key {
        "size" => Ok(format!("text-{value}")),
        "color" => Ok(format!("text-{value}")),
        "weight" => Ok(format!("font-{value}")),
        "align" => Ok(format!("text-{value}")),
        "leading" => Ok(format!("leading-{value}")),
        "tracking" => Ok(format!("tracking-{value}")),
        _ => Err(format!("unsupported text group key `{key}`")),
    }
}

fn border_child(child: &str) -> Result<String, String> {
    let Some((key, value)) = split_child_pair(child) else {
        return Ok(match child {
            "solid" | "dashed" | "dotted" | "double" | "none" => format!("border-{child}"),
            _ => format!("border-{child}"),
        });
    };

    match key {
        "width" | "color" | "style" => Ok(format!("border-{value}")),
        _ => Err(format!("unsupported border group key `{key}`")),
    }
}

fn split_child_pair(child: &str) -> Option<(&str, &str)> {
    let mut bracket_depth = 0usize;
    let mut paren_depth = 0usize;

    for (index, ch) in child.char_indices() {
        match ch {
            '[' => bracket_depth += 1,
            ']' => bracket_depth = bracket_depth.saturating_sub(1),
            '(' => paren_depth += 1,
            ')' => paren_depth = paren_depth.saturating_sub(1),
            ':' if bracket_depth == 0 && paren_depth == 0 => {
                return Some((&child[..index], &child[index + 1..]));
            }
            _ => {}
        }
    }

    None
}
