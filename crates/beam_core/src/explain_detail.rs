use crate::{
    emit::rule_selector, AtomRule, ClassToken, CompileMessage, ExplainAtom, ExplainToken,
    ResolvedAtom, RuleLayer, RuleWrapper,
};

pub(crate) fn explain_token(
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
            atoms,
            errors,
        },
        ClassToken::Group { variants, .. } => ExplainToken {
            raw: raw.to_owned(),
            kind: "group".to_owned(),
            variants: variants.clone(),
            base: None,
            atoms,
            errors,
        },
    }
}

pub(crate) fn explain_atom(atom: ResolvedAtom, rule: AtomRule) -> ExplainAtom {
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
