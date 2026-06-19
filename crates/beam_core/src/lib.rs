#![forbid(unsafe_code)]

use std::collections::BTreeSet;

mod classlist;
mod colors;
mod config;
mod declarations;
mod emit;
mod explain_detail;
mod model;
mod parser;
mod properties;
mod shortcuts;
mod typography;
mod unfold;
mod utility_groups;
mod utility_modules;
mod values;
mod variants;

pub use config::{BeamConfig, BeamPreset, BeamRecipe, BeamTokens};
pub use model::{
    ClassToken, CompileMessage, CompileResult, ExplainAtom, ExplainClassString, ExplainResult,
    ExplainToken,
};
pub use parser::{parse_classlist, parse_flat_classlist};

use model::{AtomRule, ResolvedAtom, RuleLayer, RuleWrapper, VariantEffects};

pub fn compile(config: &BeamConfig, class_strings: &[String]) -> CompileResult {
    let config = config::resolve_presets(config);
    let mut rules = BTreeSet::new();
    let mut errors = Vec::new();

    for class_string in class_strings {
        for raw in classlist::split_classlist(class_string) {
            let token = match parser::parse_token(raw) {
                Ok(token) => token,
                Err(message) => {
                    errors.push(CompileMessage {
                        class_name: raw.to_owned(),
                        message,
                    });
                    continue;
                }
            };
            let token = match shortcuts::expand_token(&config, &token) {
                Ok(token) => token,
                Err(message) => {
                    errors.push(CompileMessage {
                        class_name: raw.to_owned(),
                        message,
                    });
                    continue;
                }
            };

            for atom in unfold::unfold_token(&token, &[], raw) {
                match declarations::compile_atom(&config, &atom) {
                    Ok(rule) => {
                        rules.insert(rule);
                    }
                    Err(message) => errors.push(CompileMessage {
                        class_name: emit::canonical_class_name(&atom.variants, &atom.base),
                        message,
                    }),
                }
            }
        }
    }

    CompileResult {
        css: emit::emit_css(&config, &rules),
        warnings: Vec::new(),
        errors,
    }
}

pub fn explain(config: &BeamConfig, class_strings: &[String]) -> ExplainResult {
    let config = config::resolve_presets(config);
    let mut explained_class_strings = Vec::new();
    let mut errors = Vec::new();

    for class_string in class_strings {
        let mut tokens = Vec::new();

        for raw in classlist::split_classlist(class_string) {
            let token = match parser::parse_token(raw) {
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
                        atoms: Vec::new(),
                        errors: vec![error],
                    });
                    continue;
                }
            };
            let token = match shortcuts::expand_token(&config, &token) {
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
                        atoms: Vec::new(),
                        errors: vec![error],
                    });
                    continue;
                }
            };

            let mut token_errors = Vec::new();
            let mut explained_atoms = Vec::new();

            for atom in unfold::unfold_token(&token, &[], raw) {
                match declarations::compile_atom(&config, &atom) {
                    Ok(rule) => explained_atoms.push(explain_detail::explain_atom(atom, rule)),
                    Err(message) => {
                        let error = CompileMessage {
                            class_name: emit::canonical_class_name(&atom.variants, &atom.base),
                            message,
                        };
                        errors.push(error.clone());
                        token_errors.push(error);
                    }
                }
            }

            tokens.push(explain_detail::explain_token(
                raw,
                &token,
                explained_atoms,
                token_errors,
            ));
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

#[cfg(test)]
mod tests;
