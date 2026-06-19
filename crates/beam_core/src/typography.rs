use crate::{
    colors::resolve_color_value,
    values::{is_css_number, raw_value, token_or_raw_declaration},
    BeamConfig,
};

pub(crate) fn text_declaration(value: &str, config: &BeamConfig) -> Result<String, String> {
    match value {
        "left" | "center" | "right" => return Ok(format!("text-align:{value}")),
        _ => {}
    }

    if config.tokens.text.contains_key(value) {
        return Ok(format!("font-size:var(--text-{value})"));
    }
    if is_css_number(value) {
        return Ok(if value == "0" || value == "-0" || value == "+0" {
            "font-size:0".to_owned()
        } else {
            format!("font-size:{value}px")
        });
    }
    let css = resolve_color_value(value, &config.tokens.color)?;
    Ok(format!("color:{css}"))
}

pub(crate) fn font_declaration(config: &BeamConfig, value: &str) -> Result<String, String> {
    if let Ok(weight) = value.parse::<u16>() {
        return Ok(format!("font-weight:{weight}"));
    }

    match value {
        "thin" => Ok("font-weight:100".to_owned()),
        "extralight" => Ok("font-weight:200".to_owned()),
        "light" => Ok("font-weight:300".to_owned()),
        "normal" => Ok("font-weight:400".to_owned()),
        "medium" => Ok("font-weight:500".to_owned()),
        "semibold" => Ok("font-weight:600".to_owned()),
        "bold" => Ok("font-weight:700".to_owned()),
        "extrabold" => Ok("font-weight:800".to_owned()),
        "black" => Ok("font-weight:900".to_owned()),
        _ => token_or_raw_declaration(value, &config.tokens.font, "font-family", "font"),
    }
}

pub(crate) fn line_height_declaration(value: &str) -> Result<String, String> {
    let value = match value {
        "none" => "1".to_owned(),
        "tight" => "1.1".to_owned(),
        "snug" => "1.25".to_owned(),
        "normal" => "1.5".to_owned(),
        "relaxed" => "1.625".to_owned(),
        "loose" => "2".to_owned(),
        _ => raw_value(value)?,
    };
    Ok(format!("line-height:{value}"))
}

pub(crate) fn letter_spacing_declaration(value: &str) -> Result<String, String> {
    let value = match value {
        "tight" => "-0.025em".to_owned(),
        "normal" => "0".to_owned(),
        "wide" => "0.025em".to_owned(),
        "wider" => "0.05em".to_owned(),
        "widest" => "0.1em".to_owned(),
        _ => raw_value(value)?,
    };
    Ok(format!("letter-spacing:{value}"))
}

pub(crate) fn opacity_declaration(value: &str) -> Result<String, String> {
    let value = if let Ok(percent) = value.parse::<u16>() {
        format!("{}", f32::from(percent) / 100.0)
    } else {
        raw_value(value)?
    };
    Ok(format!("opacity:{value}"))
}

pub(crate) fn size_declaration(value: &str, property: &str) -> Result<String, String> {
    let value = match value {
        "full" => "100%".to_owned(),
        "screen" if property.contains("width") => "100vw".to_owned(),
        "screen" => "100vh".to_owned(),
        _ => raw_value(value)?,
    };
    Ok(format!("{property}:{value}"))
}

pub(crate) fn scale_declaration(value: &str) -> Result<String, String> {
    let value = if let Ok(percent) = value.parse::<u16>() {
        format!("{}", f32::from(percent) / 100.0)
    } else {
        raw_value(value)?
    };
    Ok(format!("transform:scale({value})"))
}
