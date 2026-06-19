use std::collections::BTreeMap;

pub(crate) fn token_or_raw_declaration(
    value: &str,
    map: &BTreeMap<String, String>,
    property: &str,
    family: &str,
) -> Result<String, String> {
    let value = css_value_from_map(value, map, family)?;
    Ok(format!("{property}:{value}"))
}

pub(crate) fn css_space_value(
    value: &str,
    map: &BTreeMap<String, String>,
) -> Result<String, String> {
    if let Some(raw) = dynamic_value(value) {
        return Ok(raw);
    }
    if let Some(raw) = arbitrary_value(value) {
        return Ok(raw);
    }

    if is_css_number(value) {
        return if value == "0" || value == "-0" || value == "+0" {
            Ok("0".to_owned())
        } else {
            Ok(format!("{value}px"))
        };
    }

    if !map.contains_key(value) {
        return Err(format!("space token `{value}` is not defined"));
    }
    Ok(format!("var(--space-{value})"))
}

pub(crate) fn css_value_from_map(
    value: &str,
    map: &BTreeMap<String, String>,
    family: &str,
) -> Result<String, String> {
    if let Some(raw) = dynamic_value(value) {
        return Ok(raw);
    }
    if let Some(raw) = arbitrary_value(value) {
        return Ok(raw);
    }
    if !map.contains_key(value) {
        return Err(format!("{family} token `{value}` is not defined"));
    }
    Ok(format!("var(--{family}-{value})"))
}

pub(crate) fn raw_value(value: &str) -> Result<String, String> {
    if let Some(raw) = dynamic_value(value) {
        return Ok(raw);
    }
    if let Some(raw) = arbitrary_value(value) {
        return Ok(raw);
    }
    Err(format!("unsupported raw value `{value}`"))
}

pub(crate) fn is_css_number(value: &str) -> bool {
    if value.is_empty() {
        return false;
    }
    value.parse::<f64>().is_ok()
}

pub(crate) fn arbitrary_value(value: &str) -> Option<String> {
    value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .map(|value| value.replace('_', " "))
}

pub(crate) fn dynamic_value(value: &str) -> Option<String> {
    value
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))
        .filter(|value| value.starts_with("--") && value.len() > 2)
        .map(|value| format!("var({value})"))
}
