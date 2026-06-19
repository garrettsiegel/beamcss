#![forbid(unsafe_code)]

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct BeamConfig {
    pub tokens: BeamTokens,
    /// Color token the reset paints onto `body`'s background. Names one of
    /// `tokens.color`; there are no blessed defaults.
    #[serde(default)]
    pub background: Option<String>,
    /// Color token the reset uses for `body`'s text color.
    #[serde(default)]
    pub foreground: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct BeamTokens {
    #[serde(default)]
    pub space: BTreeMap<String, String>,
    #[serde(default)]
    pub color: BTreeMap<String, String>,
    #[serde(default)]
    pub radius: BTreeMap<String, String>,
    #[serde(default)]
    pub text: BTreeMap<String, String>,
    #[serde(default)]
    pub font: BTreeMap<String, String>,
    #[serde(default)]
    pub screens: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct CompileResult {
    pub css: String,
    pub warnings: Vec<CompileMessage>,
    pub errors: Vec<CompileMessage>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct CompileMessage {
    pub class_name: String,
    pub message: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ExplainResult {
    pub class_strings: Vec<ExplainClassString>,
    pub warnings: Vec<CompileMessage>,
    pub errors: Vec<CompileMessage>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ExplainClassString {
    pub class_string: String,
    pub tokens: Vec<ExplainToken>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ExplainToken {
    pub raw: String,
    pub kind: String,
    pub variants: Vec<String>,
    pub base: Option<String>,
    pub primitive: Option<String>,
    pub atoms: Vec<ExplainAtom>,
    pub errors: Vec<CompileMessage>,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq)]
pub struct ExplainAtom {
    pub class_name: String,
    pub selector: String,
    pub variants: Vec<String>,
    pub base: String,
    pub declaration: String,
    pub layer: String,
    pub media: Vec<String>,
    pub pseudos: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassToken {
    Utility {
        variants: Vec<String>,
        base: String,
    },
    Group {
        variants: Vec<String>,
        children: Vec<ClassToken>,
    },
    Primitive {
        variants: Vec<String>,
        name: String,
        modifiers: Vec<ClassToken>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct AtomRule {
    layer: RuleLayer,
    class_name: String,
    declaration: String,
    wrappers: Vec<RuleWrapper>,
    pseudos: Vec<String>,
    selector_transform: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum RuleLayer {
    Base,
    Utilities,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum RuleWrapper {
    Media(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct ResolvedAtom {
    selector_class: String,
    variants: Vec<String>,
    base: String,
    layer: RuleLayer,
}

const PRIMITIVES: &[&str] = &["stack", "row", "cluster", "grid", "place"];

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

pub fn compile(config: &BeamConfig, class_strings: &[String]) -> CompileResult {
    let mut rules = BTreeSet::new();
    let mut errors = Vec::new();

    for class_string in class_strings {
        for raw in split_classlist(class_string) {
            let token = match parse_token(raw) {
                Ok(token) => token,
                Err(message) => {
                    errors.push(CompileMessage {
                        class_name: raw.to_owned(),
                        message,
                    });
                    continue;
                }
            };

            for atom in unfold_token(&token, &[], raw) {
                match compile_atom(config, &atom) {
                    Ok(rule) => {
                        rules.insert(rule);
                    }
                    Err(message) => errors.push(CompileMessage {
                        class_name: canonical_class_name(&atom.variants, &atom.base),
                        message,
                    }),
                }
            }
        }
    }

    CompileResult {
        css: emit_css(config, &rules),
        warnings: Vec::new(),
        errors,
    }
}

pub fn explain(config: &BeamConfig, class_strings: &[String]) -> ExplainResult {
    let mut explained_class_strings = Vec::new();
    let mut errors = Vec::new();

    for class_string in class_strings {
        let mut tokens = Vec::new();

        for raw in split_classlist(class_string) {
            let token = match parse_token(raw) {
                Ok(token) => token,
                Err(message) => {
                    let error = CompileMessage {
                        class_name: raw.to_owned(),
                        message,
                    };
                    errors.push(error.clone());
                    tokens.push(ExplainToken {
                        raw: raw.to_owned(),
                        kind: "invalid".to_owned(),
                        variants: Vec::new(),
                        base: None,
                        primitive: None,
                        atoms: Vec::new(),
                        errors: vec![error],
                    });
                    continue;
                }
            };

            let mut token_errors = Vec::new();
            let mut explained_atoms = Vec::new();

            for atom in unfold_token(&token, &[], raw) {
                match compile_atom(config, &atom) {
                    Ok(rule) => explained_atoms.push(explain_atom(atom, rule)),
                    Err(message) => {
                        let error = CompileMessage {
                            class_name: canonical_class_name(&atom.variants, &atom.base),
                            message,
                        };
                        errors.push(error.clone());
                        token_errors.push(error);
                    }
                }
            }

            tokens.push(explain_token(raw, &token, explained_atoms, token_errors));
        }

        explained_class_strings.push(ExplainClassString {
            class_string: class_string.clone(),
            tokens,
        });
    }

    ExplainResult {
        class_strings: explained_class_strings,
        warnings: Vec::new(),
        errors,
    }
}

fn explain_token(
    raw: &str,
    token: &ClassToken,
    atoms: Vec<ExplainAtom>,
    errors: Vec<CompileMessage>,
) -> ExplainToken {
    match token {
        ClassToken::Utility { variants, base } => ExplainToken {
            raw: raw.to_owned(),
            kind: "utility".to_owned(),
            variants: variants.clone(),
            base: Some(base.clone()),
            primitive: None,
            atoms,
            errors,
        },
        ClassToken::Group { variants, .. } => ExplainToken {
            raw: raw.to_owned(),
            kind: "group".to_owned(),
            variants: variants.clone(),
            base: None,
            primitive: None,
            atoms,
            errors,
        },
        ClassToken::Primitive { variants, name, .. } => ExplainToken {
            raw: raw.to_owned(),
            kind: "primitive".to_owned(),
            variants: variants.clone(),
            base: None,
            primitive: Some(name.clone()),
            atoms,
            errors,
        },
    }
}

fn explain_atom(atom: ResolvedAtom, rule: AtomRule) -> ExplainAtom {
    ExplainAtom {
        class_name: rule.class_name.clone(),
        selector: rule_selector(&rule),
        variants: atom.variants,
        base: atom.base,
        declaration: rule.declaration,
        layer: match rule.layer {
            RuleLayer::Base => "beam.base".to_owned(),
            RuleLayer::Utilities => "beam.utilities".to_owned(),
        },
        media: rule
            .wrappers
            .into_iter()
            .map(|wrapper| match wrapper {
                RuleWrapper::Media(query) => query,
            })
            .collect(),
        pseudos: rule.pseudos,
    }
}

fn parse_token(raw: &str) -> Result<ClassToken, String> {
    if raw.is_empty() {
        return Err("empty class token".to_owned());
    }

    let (variants, base) = parse_utility(raw)?;
    if PRIMITIVES.contains(&base.as_str()) {
        return Ok(ClassToken::Primitive {
            variants,
            name: base,
            modifiers: Vec::new(),
        });
    }

    if let Some(open) = top_level_group_open(raw)? {
        let close = raw
            .len()
            .checked_sub(1)
            .filter(|_| raw.ends_with(')'))
            .ok_or_else(|| "group or primitive is missing a closing `)`".to_owned())?;
        let head = &raw[..open];
        if head.ends_with('-') {
            return Ok(ClassToken::Utility { variants, base });
        }
        let inner = &raw[open + 1..close];
        if inner.trim().is_empty() {
            return Err("group or primitive has no children".to_owned());
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
            if PRIMITIVES.contains(&name.as_str()) {
                return Ok(ClassToken::Primitive {
                    variants: head_variants,
                    name,
                    modifiers: children,
                });
            }

            return Err(format!("unknown primitive `{name}`"));
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
        return Err("group or primitive is missing a head".to_owned());
    }

    if let Some(variant_head) = head.strip_suffix(':') {
        return Ok((split_variant_chain(variant_head)?, None));
    }

    let mut parts = split_variant_chain(head)?;
    let name = parts
        .pop()
        .ok_or_else(|| "primitive is missing a name".to_owned())?;
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

fn split_classlist(classlist: &str) -> Vec<&str> {
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

fn unfold_token(
    token: &ClassToken,
    inherited_variants: &[String],
    selector_class: &str,
) -> Vec<ResolvedAtom> {
    match token {
        ClassToken::Utility { variants, base } => {
            vec![ResolvedAtom {
                selector_class: selector_class.to_owned(),
                variants: concat_variants(inherited_variants, variants),
                base: base.clone(),
                layer: RuleLayer::Utilities,
            }]
        }
        ClassToken::Group { variants, children } => {
            let next = concat_variants(inherited_variants, variants);
            children
                .iter()
                .flat_map(|child| unfold_token(child, &next, selector_class))
                .collect()
        }
        ClassToken::Primitive {
            variants,
            name,
            modifiers,
        } => unfold_primitive(
            name,
            &concat_variants(inherited_variants, variants),
            modifiers,
            selector_class,
        ),
    }
}

fn unfold_primitive(
    name: &str,
    variants: &[String],
    modifiers: &[ClassToken],
    selector_class: &str,
) -> Vec<ResolvedAtom> {
    let mut atoms = match name {
        "stack" => vec![
            primitive_atom(selector_class, variants, "display-flex"),
            primitive_atom(selector_class, variants, "direction-column"),
            primitive_atom(selector_class, variants, "gap-0"),
        ],
        "row" => vec![
            primitive_atom(selector_class, variants, "display-flex"),
            primitive_atom(selector_class, variants, "direction-row"),
            primitive_atom(selector_class, variants, "gap-0"),
        ],
        "cluster" => vec![
            primitive_atom(selector_class, variants, "display-flex"),
            primitive_atom(selector_class, variants, "wrap"),
            primitive_atom(selector_class, variants, "align-center"),
            primitive_atom(selector_class, variants, "gap-0"),
        ],
        "grid" => vec![primitive_atom(selector_class, variants, "display-grid")],
        "place" => vec![
            primitive_atom(selector_class, variants, "display-grid"),
            primitive_atom(selector_class, variants, "place-center"),
        ],
        _ => Vec::new(),
    };

    atoms.extend(
        modifiers
            .iter()
            .flat_map(|modifier| unfold_primitive_modifier(modifier, variants, selector_class)),
    );
    atoms
}

fn unfold_primitive_modifier(
    token: &ClassToken,
    inherited_variants: &[String],
    selector_class: &str,
) -> Vec<ResolvedAtom> {
    unfold_token(token, inherited_variants, selector_class)
        .into_iter()
        .map(|atom| ResolvedAtom {
            selector_class: atom.selector_class,
            variants: atom.variants,
            base: primitive_modifier_base(&atom.base),
            layer: RuleLayer::Base,
        })
        .collect()
}

fn primitive_modifier_base(base: &str) -> String {
    match base {
        "center" => "align-center".to_owned(),
        "between" => "justify-between".to_owned(),
        "around" => "justify-around".to_owned(),
        "evenly" => "justify-evenly".to_owned(),
        "wrap" | "nowrap" => base.to_owned(),
        _ => base.to_owned(),
    }
}

fn primitive_atom(selector_class: &str, variants: &[String], base: &str) -> ResolvedAtom {
    ResolvedAtom {
        selector_class: selector_class.to_owned(),
        variants: variants.to_vec(),
        base: base.to_owned(),
        layer: RuleLayer::Base,
    }
}

fn concat_variants(left: &[String], right: &[String]) -> Vec<String> {
    let mut variants = left.to_vec();
    variants.extend(right.iter().cloned());
    variants
}

fn compile_atom(config: &BeamConfig, atom: &ResolvedAtom) -> Result<AtomRule, String> {
    let declaration = declaration_for_base(config, &atom.base)?;
    let class_name = if atom.selector_class.is_empty() {
        canonical_class_name(&atom.variants, &atom.base)
    } else {
        atom.selector_class.clone()
    };
    let (wrappers, pseudos, selector_transform) = variant_effects(config, &atom.variants)?;

    Ok(AtomRule {
        layer: atom.layer,
        class_name,
        declaration,
        wrappers,
        pseudos,
        selector_transform,
    })
}

fn declaration_for_base(config: &BeamConfig, base: &str) -> Result<String, String> {
    if let Some(value) = base.strip_prefix("inset-x-") {
        let value = raw_value(value)?;
        return Ok(format!("left:{value};right:{value}"));
    }
    if let Some(value) = base.strip_prefix("inset-y-") {
        let value = raw_value(value)?;
        return Ok(format!("top:{value};bottom:{value}"));
    }
    if let Some(value) = base.strip_prefix("inset-") {
        return raw_declaration(value, "inset");
    }
    if let Some(value) = base.strip_prefix("top-") {
        return raw_declaration(value, "top");
    }
    if let Some(value) = base.strip_prefix("right-") {
        return raw_declaration(value, "right");
    }
    if let Some(value) = base.strip_prefix("bottom-") {
        return raw_declaration(value, "bottom");
    }
    if let Some(value) = base.strip_prefix("left-") {
        return raw_declaration(value, "left");
    }
    if let Some(value) = base.strip_prefix("z-") {
        return raw_declaration(value, "z-index");
    }
    if let Some(value) = base.strip_prefix("gap-x-") {
        return space_declaration(config, value, "column-gap");
    }
    if let Some(value) = base.strip_prefix("gap-y-") {
        return space_declaration(config, value, "row-gap");
    }
    if let Some(value) = base.strip_prefix("gap-") {
        return space_declaration(config, value, "gap");
    }

    if let Some(value) = base.strip_prefix("bg-") {
        return color_declaration(value, &config.tokens.color, "background");
    }
    if let Some(value) = base.strip_prefix("fg-") {
        return color_declaration(value, &config.tokens.color, "color");
    }
    if let Some(value) = base.strip_prefix("bd-") {
        return color_declaration(value, &config.tokens.color, "border-color");
    }
    if let Some(value) = base.strip_prefix("round-") {
        return token_or_raw_declaration(value, &config.tokens.radius, "border-radius", "radius");
    }
    if let Some(value) = base.strip_prefix("text-") {
        return token_or_raw_declaration(value, &config.tokens.text, "font-size", "text");
    }
    if let Some(value) = base.strip_prefix("font-") {
        return font_declaration(config, value);
    }
    if let Some(value) = base.strip_prefix("leading-") {
        return line_height_declaration(value);
    }
    if let Some(value) = base.strip_prefix("tracking-") {
        return letter_spacing_declaration(value);
    }
    if let Some(value) = base.strip_prefix("opacity-") {
        return opacity_declaration(value);
    }
    if let Some(value) = base.strip_prefix("shadow-") {
        return raw_declaration(value, "box-shadow");
    }
    if let Some((property, value)) = spacing_property(base, "p") {
        return space_declaration(config, value, property);
    }
    if let Some((property, value)) = spacing_property(base, "m") {
        return space_declaration(config, value, property);
    }
    if let Some((property, value)) = size_property(base) {
        return size_declaration(value, property);
    }
    if let Some(value) = base.strip_prefix("scale-") {
        return scale_declaration(value);
    }
    if let Some(value) = base.strip_prefix("cols-") {
        return grid_track_declaration(value, "grid-template-columns");
    }
    if let Some(value) = base.strip_prefix("rows-") {
        return grid_track_declaration(value, "grid-template-rows");
    }

    match base {
        "display-flex" => Ok("display:flex".to_owned()),
        "display-grid" => Ok("display:grid".to_owned()),
        "inline-block" => Ok("display:inline-block".to_owned()),
        "direction-column" => Ok("flex-direction:column".to_owned()),
        "direction-row" => Ok("flex-direction:row".to_owned()),
        "place-center" => Ok("place-items:center".to_owned()),
        "wrap" => Ok("flex-wrap:wrap".to_owned()),
        "nowrap" => Ok("flex-wrap:nowrap".to_owned()),
        "align-start" => Ok("align-items:flex-start".to_owned()),
        "align-end" => Ok("align-items:flex-end".to_owned()),
        "align-stretch" => Ok("align-items:stretch".to_owned()),
        "align-baseline" => Ok("align-items:baseline".to_owned()),
        "align-center" => Ok("align-items:center".to_owned()),
        "justify-start" => Ok("justify-content:flex-start".to_owned()),
        "justify-center" => Ok("justify-content:center".to_owned()),
        "justify-end" => Ok("justify-content:flex-end".to_owned()),
        "justify-between" => Ok("justify-content:space-between".to_owned()),
        "justify-around" => Ok("justify-content:space-around".to_owned()),
        "justify-evenly" => Ok("justify-content:space-evenly".to_owned()),
        "block" => Ok("display:block".to_owned()),
        "hidden" => Ok("display:none".to_owned()),
        "absolute" => Ok("position:absolute".to_owned()),
        "relative" => Ok("position:relative".to_owned()),
        "fixed" => Ok("position:fixed".to_owned()),
        "sticky" => Ok("position:sticky".to_owned()),
        "overflow-hidden" => Ok("overflow:hidden".to_owned()),
        "overflow-auto" => Ok("overflow:auto".to_owned()),
        "overflow-x-auto" => Ok("overflow-x:auto".to_owned()),
        "overflow-y-auto" => Ok("overflow-y:auto".to_owned()),
        "list-none" => Ok("list-style:none".to_owned()),
        "uppercase" => Ok("text-transform:uppercase".to_owned()),
        "text-left" => Ok("text-align:left".to_owned()),
        "text-center" => Ok("text-align:center".to_owned()),
        "text-right" => Ok("text-align:right".to_owned()),
        "no-underline" => Ok("text-decoration:none".to_owned()),
        "cursor-pointer" => Ok("cursor:pointer".to_owned()),
        "border" => Ok("border-width:1px;border-style:solid".to_owned()),
        "border-0" => Ok("border-width:0".to_owned()),
        "border-t" => Ok("border-top-width:1px;border-top-style:solid".to_owned()),
        "border-b" => Ok("border-bottom-width:1px;border-bottom-style:solid".to_owned()),
        "border-l" => Ok("border-left-width:1px;border-left-style:solid".to_owned()),
        "border-r" => Ok("border-right-width:1px;border-right-style:solid".to_owned()),
        _ => {
            if let Some(value) = base.strip_prefix("border-") {
                return Ok(format!("border-width:{value}px;border-style:solid"));
            }
            Err(format!("unsupported utility `{base}`"))
        }
    }
}

fn raw_declaration(value: &str, property: &str) -> Result<String, String> {
    let value = raw_value(value)?;
    Ok(format!("{property}:{value}"))
}

fn spacing_property<'a>(class_name: &'a str, family: &str) -> Option<(&'static str, &'a str)> {
    let mappings = match family {
        "p" => [
            ("px-", "padding-inline"),
            ("py-", "padding-block"),
            ("pt-", "padding-top"),
            ("pr-", "padding-right"),
            ("pb-", "padding-bottom"),
            ("pl-", "padding-left"),
            ("p-", "padding"),
        ],
        "m" => [
            ("mx-", "margin-inline"),
            ("my-", "margin-block"),
            ("mt-", "margin-top"),
            ("mr-", "margin-right"),
            ("mb-", "margin-bottom"),
            ("ml-", "margin-left"),
            ("m-", "margin"),
        ],
        _ => return None,
    };

    mappings.iter().find_map(|(prefix, property)| {
        class_name
            .strip_prefix(prefix)
            .map(|value| (*property, value))
    })
}

fn size_property(class_name: &str) -> Option<(&'static str, &str)> {
    [
        ("min-w-", "min-width"),
        ("min-h-", "min-height"),
        ("max-w-", "max-width"),
        ("max-h-", "max-height"),
        ("w-", "width"),
        ("h-", "height"),
    ]
    .iter()
    .find_map(|(prefix, property)| {
        class_name
            .strip_prefix(prefix)
            .map(|value| (*property, value))
    })
}

fn space_declaration(config: &BeamConfig, value: &str, property: &str) -> Result<String, String> {
    let value = css_space_value(value, &config.tokens.space)?;
    Ok(format!("{property}:{value}"))
}

fn token_or_raw_declaration(
    value: &str,
    map: &BTreeMap<String, String>,
    property: &str,
    family: &str,
) -> Result<String, String> {
    let value = css_value_from_map(value, map, family)?;
    Ok(format!("{property}:{value}"))
}

fn color_declaration(
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
fn resolve_color_value(value: &str, map: &BTreeMap<String, String>) -> Result<String, String> {
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

fn font_declaration(config: &BeamConfig, value: &str) -> Result<String, String> {
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

fn line_height_declaration(value: &str) -> Result<String, String> {
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

fn letter_spacing_declaration(value: &str) -> Result<String, String> {
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

fn opacity_declaration(value: &str) -> Result<String, String> {
    let value = if let Ok(percent) = value.parse::<u16>() {
        format!("{}", f32::from(percent) / 100.0)
    } else {
        raw_value(value)?
    };
    Ok(format!("opacity:{value}"))
}

fn size_declaration(value: &str, property: &str) -> Result<String, String> {
    let value = match value {
        "full" => "100%".to_owned(),
        "screen" if property.contains("width") => "100vw".to_owned(),
        "screen" => "100vh".to_owned(),
        _ => raw_value(value)?,
    };
    Ok(format!("{property}:{value}"))
}

fn scale_declaration(value: &str) -> Result<String, String> {
    let value = if let Ok(percent) = value.parse::<u16>() {
        format!("{}", f32::from(percent) / 100.0)
    } else {
        raw_value(value)?
    };
    Ok(format!("transform:scale({value})"))
}

fn grid_track_declaration(value: &str, property: &str) -> Result<String, String> {
    let value = if let Ok(count) = value.parse::<usize>() {
        format!("repeat({count},1fr)")
    } else {
        raw_value(value)?
    };
    Ok(format!("{property}:{value}"))
}

fn css_space_value(value: &str, map: &BTreeMap<String, String>) -> Result<String, String> {
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

fn css_value_from_map(
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

fn raw_value(value: &str) -> Result<String, String> {
    if let Some(raw) = dynamic_value(value) {
        return Ok(raw);
    }
    if let Some(raw) = arbitrary_value(value) {
        return Ok(raw);
    }
    Err(format!("unsupported raw value `{value}`"))
}

fn is_css_number(value: &str) -> bool {
    if value.is_empty() {
        return false;
    }
    value.parse::<f64>().is_ok()
}

fn arbitrary_value(value: &str) -> Option<String> {
    value
        .strip_prefix('[')
        .and_then(|value| value.strip_suffix(']'))
        .map(|value| value.replace('_', " "))
}

fn dynamic_value(value: &str) -> Option<String> {
    value
        .strip_prefix('(')
        .and_then(|value| value.strip_suffix(')'))
        .filter(|value| value.starts_with("--") && value.len() > 2)
        .map(|value| format!("var({value})"))
}

fn variant_effects(
    config: &BeamConfig,
    variants: &[String],
) -> Result<(Vec<RuleWrapper>, Vec<String>, Option<String>), String> {
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

fn canonical_class_name(variants: &[String], base: &str) -> String {
    if variants.is_empty() {
        base.to_owned()
    } else {
        format!("{}:{base}", variants.join(":"))
    }
}

fn emit_css(config: &BeamConfig, rules: &BTreeSet<AtomRule>) -> String {
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

    emit_token_map(&mut css, "space", &config.tokens.space);
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

fn rule_selector(rule: &AtomRule) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;

    fn config() -> BeamConfig {
        BeamConfig {
            tokens: BeamTokens {
                space: BTreeMap::from([("card".into(), "1.25rem".into())]),
                color: BTreeMap::from([
                    ("accent".into(), "#3b82f6".into()),
                    ("on-accent".into(), "#ffffff".into()),
                    ("surface".into(), "#16161a".into()),
                ]),
                radius: BTreeMap::from([("md".into(), "8px".into())]),
                text: BTreeMap::from([("lg".into(), "20px".into())]),
                font: BTreeMap::from([("ui".into(), "Inter, system-ui, sans-serif".into())]),
                screens: BTreeMap::from([
                    ("tablet".into(), "48rem".into()),
                    ("desktop".into(), "64rem".into()),
                    (
                        "mobile-landscape".into(),
                        "(max-width:47.999rem) and (orientation:landscape)".into(),
                    ),
                ]),
            },
            background: Some("base".into()),
            foreground: Some("fg".into()),
        }
    }

    #[test]
    fn emits_tokens_and_utilities() {
        let result = compile(
            &config(),
            &["p-1 px-2 gap-card bg-surface fg-accent round-md text-lg".into()],
        );

        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert!(result.css.contains("@layer beam.reset{"));
        assert!(result.css.contains("body{min-width:320px;min-height:100vh;margin:0;background:var(--color-base);color:var(--color-fg);}"));
        assert!(result.css.contains("--space-card:1.25rem;"));
        assert!(result
            .css
            .contains(".bg-surface{background:var(--color-surface);}"));
        assert!(result
            .css
            .contains(".fg-accent{color:var(--color-accent);}"));
        assert!(result.css.contains(".gap-card{gap:var(--space-card);}"));
        assert!(result.css.contains(".p-1{padding:1px;}"));
        assert!(result.css.contains(".px-2{padding-inline:2px;}"));
        assert!(result
            .css
            .contains(".round-md{border-radius:var(--radius-md);}"));
        assert!(result.css.contains(".text-lg{font-size:var(--text-lg);}"));
    }

    #[test]
    fn color_algebra_alpha_shade_and_mix() {
        let result = compile(
            &config(),
            &["bg-surface bg-accent/50 fg-accent+10 bg-surface-20 bg-accent~surface bg-on-accent bg-on-accent-10 bg-accent-10/50".into()],
        );

        assert!(result.errors.is_empty(), "{:?}", result.errors);
        // plain token
        assert!(result
            .css
            .contains(".bg-surface{background:var(--color-surface);}"));
        // alpha
        assert!(result.css.contains(
            ".bg-accent\\/50{background:color-mix(in oklab,var(--color-accent) 50%,transparent);}"
        ));
        // lighten
        assert!(result.css.contains(
            ".fg-accent\\+10{color:color-mix(in oklab,var(--color-accent),white 10%);}"
        ));
        // darken
        assert!(result.css.contains(
            ".bg-surface-20{background:color-mix(in oklab,var(--color-surface),black 20%);}"
        ));
        // mix two tokens
        assert!(result.css.contains(
            ".bg-accent\\~surface{background:color-mix(in oklab,var(--color-accent),var(--color-surface));}"
        ));
        // hyphenated token stays intact (longest match wins)
        assert!(result
            .css
            .contains(".bg-on-accent{background:var(--color-on-accent);}"));
        // darken applied to a hyphenated token
        assert!(result.css.contains(
            ".bg-on-accent-10{background:color-mix(in oklab,var(--color-on-accent),black 10%);}"
        ));
        // shade composed with alpha
        assert!(result.css.contains(
            ".bg-accent-10\\/50{background:color-mix(in oklab,color-mix(in oklab,var(--color-accent),black 10%) 50%,transparent);}"
        ));
    }

    #[test]
    fn color_algebra_arbitrary_and_dynamic_pass_through() {
        let result = compile(
            &config(),
            &["bg-[oklch(72%_0.14_240)] fg-(--brand)".into()],
        );

        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert!(result
            .css
            .contains("background:oklch(72% 0.14 240);"));
        assert!(result.css.contains("color:var(--brand);"));
    }

    #[test]
    fn color_algebra_reports_undefined_token() {
        let result = compile(&config(), &["bg-missing fg-accent+bogus".into()]);
        assert_eq!(result.errors.len(), 2);
        assert_eq!(result.errors[0].message, "color token `missing` is not defined");
        assert_eq!(result.errors[1].message, "color token `accent+bogus` is not defined");
    }

    #[test]
    fn reset_paints_body_from_config_pointers() {
        let mut cfg = config();
        cfg.tokens.color.insert("page".into(), "#000".into());
        cfg.background = Some("page".into());
        cfg.foreground = Some("surface".into());
        let result = compile(&cfg, &[]);
        assert!(result.css.contains(
            "body{min-width:320px;min-height:100vh;margin:0;background:var(--color-page);color:var(--color-surface);}"
        ));
    }

    #[test]
    fn reset_omits_body_color_when_pointers_unset() {
        let mut cfg = config();
        cfg.background = None;
        cfg.foreground = None;
        let result = compile(&cfg, &[]);
        assert!(result
            .css
            .contains("body{min-width:320px;min-height:100vh;margin:0;}"));
    }

    #[test]
    fn dedupes_repeated_atoms() {
        let result = compile(&config(), &["p-1 p-1".into(), "p-1".into()]);

        assert!(result.errors.is_empty());
        assert_eq!(result.css.matches(".p-1{").count(), 1);
    }

    #[test]
    fn unfolds_grouped_and_nested_variants() {
        let result = compile(
            &config(),
            &["tablet:(p-6 round-md hover:(bg-surface fg-on-accent))".into()],
        );

        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert!(result.css.contains(
            "@media (min-width:48rem){.tablet\\:\\(p-6.round-md.hover\\:\\(bg-surface.fg-on-accent\\)\\):hover{background:var(--color-surface);}}\n"
        ));
        assert!(result.css.contains(
            "@media (min-width:48rem){.tablet\\:\\(p-6.round-md.hover\\:\\(bg-surface.fg-on-accent\\)\\):hover{color:var(--color-on-accent);}}\n"
        ));
    }

    #[test]
    fn compiles_layout_primitives() {
        let result = compile(
            &config(),
            &[
                "stack(center gap-4) tablet:row(between center) grid(cols-1 tablet:cols-3 gap-6)"
                    .into(),
            ],
        );

        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert!(result
            .css
            .contains(".stack\\(center.gap-4\\){display:flex;}"));
        assert!(result
            .css
            .contains(".stack\\(center.gap-4\\){flex-direction:column;}"));
        assert!(result
            .css
            .contains(".stack\\(center.gap-4\\){align-items:center;}"));
        assert!(result.css.contains(
            "@media (min-width:48rem){.tablet\\:row\\(between.center\\){flex-direction:row;}}\n"
        ));
        assert!(result.css.contains(
            ".grid\\(cols-1.tablet\\:cols-3.gap-6\\){grid-template-columns:repeat(1,1fr);}"
        ));
        assert!(result.css.contains(
            "@media (min-width:48rem){.grid\\(cols-1.tablet\\:cols-3.gap-6\\){grid-template-columns:repeat(3,1fr);}}\n"
        ));
    }

    #[test]
    fn emits_responsive_primitive_modifiers_after_base_modifiers() {
        let result = compile(
            &config(),
            &["grid(cols-1 desktop:cols-[0.58fr_0.42fr] gap-4)".into()],
        );

        assert!(result.errors.is_empty(), "{:?}", result.errors);
        let base = result
            .css
            .find(".grid\\(cols-1.desktop\\:cols-\\[0\\.58fr_0\\.42fr\\].gap-4\\){grid-template-columns:repeat(1,1fr);}")
            .expect("base grid-template-columns rule should be emitted");
        let responsive = result
            .css
            .find("@media (min-width:64rem){.grid\\(cols-1.desktop\\:cols-\\[0\\.58fr_0\\.42fr\\].gap-4\\){grid-template-columns:0.58fr 0.42fr;}}")
            .expect("responsive grid-template-columns rule should be emitted");

        assert!(
            base < responsive,
            "responsive primitive modifiers must be emitted after base modifiers so they win in the cascade"
        );
    }

    #[test]
    fn compiles_arbitrary_and_dynamic_values() {
        let result = compile(
            &config(),
            &[
                "w-[347px] grid(cols-[200px_1fr]) w-(--w) w-screen h-screen bg-[red] p-0 p-[1rem] gap-(--gap) scale-105"
                    .into(),
            ],
        );

        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert!(result.css.contains(".w-\\[347px\\]{width:347px;}"));
        assert!(result
            .css
            .contains(".grid\\(cols-\\[200px_1fr\\]\\){grid-template-columns:200px 1fr;}"));
        assert!(result.css.contains(".w-\\(--w\\){width:var(--w);}"));
        assert!(result.css.contains(".w-screen{width:100vw;}"));
        assert!(result.css.contains(".h-screen{height:100vh;}"));
        assert!(result.css.contains(".bg-\\[red\\]{background:red;}"));
        assert!(result.css.contains(".p-0{padding:0;}"));
        assert!(result.css.contains(".p-\\[1rem\\]{padding:1rem;}"));
        assert!(result.css.contains(".gap-\\(--gap\\){gap:var(--gap);}"));
        assert!(result.css.contains(".scale-105{transform:scale(1.05);}"));
    }

    #[test]
    fn compiles_modern_color_syntaxes() {
        let result = compile(
            &config(),
            &[
                "bg-[#fff] fg-[rgb(255_255_255_/_80%)] bg-[hsl(220_80%_56%)] fg-[oklch(72%_0.14_240)] bg-[color(display-p3_0.2_0.7_0.5)] bd-[color-mix(in_srgb,var(--color-surface),white_8%)]".into(),
            ],
        );

        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert!(result.css.contains(".bg-\\[\\#fff\\]{background:#fff;}"));
        assert!(result
            .css
            .contains(".fg-\\[rgb\\(255_255_255_\\/_80\\%\\)\\]{color:rgb(255 255 255 / 80%);}"));
        assert!(result
            .css
            .contains(".fg-\\[oklch\\(72\\%_0\\.14_240\\)\\]{color:oklch(72% 0.14 240);}"));
        assert!(result
            .css
            .contains("background:color(display-p3 0.2 0.7 0.5);"));
        assert!(result
            .css
            .contains("border-color:color-mix(in srgb,var(--color-surface),white 8%);"));
    }

    #[test]
    fn compiles_named_and_full_media_queries() {
        let result = compile(
            &config(),
            &["tablet:p-4 desktop:p-8 mobile-landscape:gap-4".into()],
        );

        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert!(result
            .css
            .contains("@media (min-width:48rem){.tablet\\:p-4{padding:4px;}}"));
        assert!(result
            .css
            .contains("@media (min-width:64rem){.desktop\\:p-8{padding:8px;}}"));
        assert!(result.css.contains(
            "@media (max-width:47.999rem) and (orientation:landscape){.mobile-landscape\\:gap-4{gap:4px;}}"
        ));
    }

    #[test]
    fn compiles_homepage_utility_surface() {
        let result = compile(
            &config(),
            &[
                "relative absolute overflow-hidden overflow-x-auto z-[1] inset-x-[0] top-[3rem] h-[12rem]".into(),
                "leading-tight tracking-widest uppercase no-underline cursor-pointer shadow-[0_24px_80px_rgb(0_0_0_/_22%)] opacity-75".into(),
                "border border-2 border-t border-b bd-[rgb(247_244_237_/_12%)] bg-[linear-gradient(120deg,rgb(27_37_34_/_92%),#111414)]".into(),
            ],
        );

        assert!(result.errors.is_empty(), "{:?}", result.errors);
        assert!(result.css.contains(".overflow-hidden{overflow:hidden;}"));
        assert!(result.css.contains(".z-\\[1\\]{z-index:1;}"));
        assert!(result.css.contains(".leading-tight{line-height:1.1;}"));
        assert!(result
            .css
            .contains(".tracking-widest{letter-spacing:0.1em;}"));
        assert!(result.css.contains(".uppercase{text-transform:uppercase;}"));
        assert!(result.css.contains(".opacity-75{opacity:0.75;}"));
        assert!(result
            .css
            .contains(".border{border-width:1px;border-style:solid;}"));
        assert!(result
            .css
            .contains(".border-2{border-width:2px;border-style:solid;}"));
    }

    #[test]
    fn reports_unsupported_utilities_without_panicking() {
        let result = compile(&config(), &["unknown".into()]);

        assert_eq!(result.errors.len(), 1);
        assert_eq!(
            result.errors[0].message,
            "unsupported utility `unknown`".to_owned()
        );
    }

    #[test]
    fn parser_rejects_malformed_groups() {
        let errors = parse_classlist("hover:(bg-accent").unwrap_err();

        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].message, "unclosed `(` in utility");
    }
}
