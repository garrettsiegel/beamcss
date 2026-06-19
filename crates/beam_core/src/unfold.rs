use crate::{ClassToken, ResolvedAtom, RuleLayer};

pub(crate) fn unfold_token(
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
    }
}

fn concat_variants(left: &[String], right: &[String]) -> Vec<String> {
    let mut variants = left.to_vec();
    variants.extend(right.iter().cloned());
    variants
}
