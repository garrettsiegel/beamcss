use crate::{BeamConfig, RuleWrapper, VariantEffects};

pub(crate) fn variant_effects(
    config: &BeamConfig,
    variants: &[String],
) -> Result<VariantEffects, String> {
    let mut wrappers = Vec::new();
    let mut pseudos = Vec::new();
    let mut selector_transform = None;

    for variant in variants {
        if let Some(screen) = config.tokens.screens.get(variant) {
            wrappers.push(RuleWrapper::Media(screen_media_query(screen)));
            continue;
        }

        match variant.as_str() {
            "hover" => pseudos.push(":hover".to_owned()),
            "focus" => pseudos.push(":focus".to_owned()),
            "focus-visible" => pseudos.push(":focus-visible".to_owned()),
            "focus-within" => pseudos.push(":focus-within".to_owned()),
            "active" => pseudos.push(":active".to_owned()),
            "disabled" => pseudos.push(":disabled".to_owned()),
            "first" => pseudos.push(":first-child".to_owned()),
            "last" => pseudos.push(":last-child".to_owned()),
            "odd" => pseudos.push(":nth-child(odd)".to_owned()),
            "even" => pseudos.push(":nth-child(even)".to_owned()),
            "dark" => selector_transform = Some("[data-theme=\"dark\"] &".to_owned()),
            "group-hover" => selector_transform = Some(".group:hover &".to_owned()),
            "group-focus" => selector_transform = Some(".group:focus &".to_owned()),
            "peer-checked" => selector_transform = Some(".peer:checked ~ &".to_owned()),
            "peer-focus" => selector_transform = Some(".peer:focus ~ &".to_owned()),
            "motion-safe" => wrappers.push(RuleWrapper::Media(
                "(prefers-reduced-motion:no-preference)".to_owned(),
            )),
            "print" => wrappers.push(RuleWrapper::Media("print".to_owned())),
            _ if variant.starts_with('[') && variant.ends_with(']') => {
                selector_transform = Some(variant[1..variant.len() - 1].to_owned());
            }
            _ => return Err(format!("unsupported variant `{variant}`")),
        }
    }

    Ok((wrappers, pseudos, selector_transform))
}

fn screen_media_query(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.starts_with('(') || is_media_query_keyword(trimmed) {
        trimmed.to_owned()
    } else {
        format!("(min-width:{trimmed})")
    }
}

fn is_media_query_keyword(value: &str) -> bool {
    ["all", "not", "only", "print", "screen", "speech"]
        .iter()
        .any(|keyword| value == *keyword || value.starts_with(&format!("{keyword} ")))
}
