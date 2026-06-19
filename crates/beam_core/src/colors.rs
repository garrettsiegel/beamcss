use std::collections::BTreeMap;

use crate::values::{arbitrary_value, dynamic_value};

pub(crate) fn color_declaration(
    value: &str,
    map: &BTreeMap<String, String>,
    property: &str,
) -> Result<String, String> {
    let css = resolve_color_value(value, map)?;
    Ok(format!("{property}:{css}"))
}

/// Resolve a color value, applying Beam's color algebra to tokens:
/// `brand` (token), `brand/50` (alpha), `brand+10` / `brand-10`
/// (lighten / darken), `brand~ink` (mix two tokens). Arbitrary `[...]` and
/// dynamic `(--x)` values are escape hatches passed through untouched.
pub(crate) fn resolve_color_value(
    value: &str,
    map: &BTreeMap<String, String>,
) -> Result<String, String> {
    if let Some(raw) = dynamic_value(value) {
        return Ok(raw);
    }
    if let Some(raw) = arbitrary_value(value) {
        return Ok(raw);
    }

    let (color, alpha) = match value.split_once('/') {
        Some((color, alpha)) => (color, Some(parse_color_amount(alpha, "alpha")?)),
        None => (value, None),
    };

    let base = resolve_color_expr(color, map)?;

    Ok(match alpha {
        Some(percent) => format!("color-mix(in oklab,{base} {percent}%,transparent)"),
        None => base,
    })
}

fn resolve_color_expr(expr: &str, map: &BTreeMap<String, String>) -> Result<String, String> {
    if let Some((left, right)) = expr.split_once('~') {
        let left = resolve_color_token(left, map)?;
        let right = resolve_color_token(right, map)?;
        return Ok(format!("color-mix(in oklab,{left},{right})"));
    }

    // Longest match wins: a defined token (e.g. `on-accent`) is preferred over
    // reading a trailing `-N` as a darken shade, so hyphenated names stay intact.
    if map.contains_key(expr) {
        return Ok(format!("var(--color-{expr})"));
    }

    if let Some((name, sign, amount)) = split_shade(expr) {
        if map.contains_key(name) {
            let amount = parse_color_amount(amount, "shade")?;
            let mixer = if sign == '+' { "white" } else { "black" };
            return Ok(format!(
                "color-mix(in oklab,var(--color-{name}),{mixer} {amount}%)"
            ));
        }
    }

    Err(format!("color token `{expr}` is not defined"))
}

fn resolve_color_token(name: &str, map: &BTreeMap<String, String>) -> Result<String, String> {
    if map.contains_key(name) {
        Ok(format!("var(--color-{name})"))
    } else {
        Err(format!("color token `{name}` is not defined"))
    }
}

/// Split a trailing `+N` / `-N` shade suffix off a color expression. The sign
/// must be followed by digits only, so a hyphen inside a token name (`on-accent`)
/// is never mistaken for a darken op.
fn split_shade(expr: &str) -> Option<(&str, char, &str)> {
    for (index, ch) in expr.char_indices() {
        if (ch == '+' || ch == '-') && index > 0 {
            let rest = &expr[index + 1..];
            if !rest.is_empty() && rest.bytes().all(|byte| byte.is_ascii_digit()) {
                return Some((&expr[..index], ch, rest));
            }
        }
    }
    None
}

fn parse_color_amount(value: &str, kind: &str) -> Result<u16, String> {
    let amount: u16 = value
        .parse()
        .map_err(|_| format!("invalid {kind} amount `{value}`"))?;
    if amount > 100 {
        return Err(format!("{kind} amount `{value}` must be 0-100"));
    }
    Ok(amount)
}
