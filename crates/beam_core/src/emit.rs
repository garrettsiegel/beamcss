use std::collections::{BTreeMap, BTreeSet};

use crate::{AtomRule, BeamConfig, RuleLayer, RuleWrapper};

pub(crate) fn canonical_class_name(variants: &[String], base: &str) -> String {
    if variants.is_empty() {
        base.to_owned()
    } else {
        format!("{}:{base}", variants.join(":"))
    }
}

pub(crate) fn emit_css(config: &BeamConfig, rules: &BTreeSet<AtomRule>) -> String {
    let mut css = String::from("@layer beam.reset, beam.tokens, beam.base, beam.utilities;\n");
    let mut body = String::from("min-width:320px;min-height:100vh;margin:0;");
    if let Some(token) = &config.background {
        body.push_str(&format!("background:var(--color-{token});"));
    }
    if let Some(token) = &config.foreground {
        body.push_str(&format!("color:var(--color-{token});"));
    }
    css.push_str("@layer beam.reset{\n*,::before,::after{box-sizing:border-box;}\nhtml{font-family:var(--font-ui);font-synthesis:none;text-rendering:optimizeLegibility;-webkit-font-smoothing:antialiased;-moz-osx-font-smoothing:grayscale;}\nbody{");
    css.push_str(&body);
    css.push_str("}\nbutton,input,textarea,select{font:inherit;}\na{color:inherit;}\nh1,h2,h3,p,figure,pre,ul{margin:0;}\nbutton{border:0;}\n}\n");
    css.push_str("@layer beam.tokens{\n:root{\n");

    emit_token_map(&mut css, "space", &config.tokens.spacing);
    emit_token_map(&mut css, "color", &config.tokens.color);
    emit_token_map(&mut css, "radius", &config.tokens.radius);
    emit_token_map(&mut css, "text", &config.tokens.text);
    emit_token_map(&mut css, "font", &config.tokens.font);
    emit_token_map(&mut css, "screen", &config.tokens.screens);

    css.push_str("}\n}\n@layer beam.base{\n");

    emit_layer_rules(&mut css, rules, RuleLayer::Base);

    css.push_str("}\n@layer beam.utilities{\n");

    emit_layer_rules(&mut css, rules, RuleLayer::Utilities);

    css.push_str("}\n");
    css
}

fn emit_token_map(css: &mut String, family: &str, map: &BTreeMap<String, String>) {
    for (name, value) in map {
        css.push_str(&format!("--{family}-{name}:{value};\n"));
    }
}

fn emit_layer_rules(css: &mut String, rules: &BTreeSet<AtomRule>, layer: RuleLayer) {
    for rule in rules
        .iter()
        .filter(|rule| rule.layer == layer && rule.wrappers.is_empty())
    {
        emit_rule(css, rule, 0);
    }

    for rule in rules
        .iter()
        .filter(|rule| rule.layer == layer && !rule.wrappers.is_empty())
    {
        emit_rule(css, rule, 0);
    }
}

fn emit_rule(css: &mut String, rule: &AtomRule, wrapper_index: usize) {
    if let Some(wrapper) = rule.wrappers.get(wrapper_index) {
        match wrapper {
            RuleWrapper::Media(query) => {
                css.push_str(&format!("@media {query}{{"));
                emit_rule(css, rule, wrapper_index + 1);
                css.push_str("}\n");
            }
        }
        return;
    }

    css.push_str(&format!("{}{{{};}}", rule_selector(rule), rule.declaration));
    if rule.wrappers.is_empty() {
        css.push('\n');
    }
}

pub(crate) fn rule_selector(rule: &AtomRule) -> String {
    let base_selector = class_name_selector(&rule.class_name);
    let selector_with_pseudos = format!("{}{}", base_selector, rule.pseudos.join(""));
    rule.selector_transform
        .as_ref()
        .map(|transform| transform.replace('&', &selector_with_pseudos))
        .unwrap_or(selector_with_pseudos)
}

fn class_name_selector(class_name: &str) -> String {
    class_name
        .split_whitespace()
        .map(|part| format!(".{}", escape_class_selector(part)))
        .collect::<Vec<_>>()
        .join("")
}

fn escape_class_selector(class_name: &str) -> String {
    class_name
        .chars()
        .flat_map(|ch| match ch {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' => vec![ch],
            _ => vec!['\\', ch],
        })
        .collect()
}
