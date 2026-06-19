use serde::{Deserialize, Serialize};

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
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct AtomRule {
    pub(crate) layer: RuleLayer,
    pub(crate) class_name: String,
    pub(crate) declaration: String,
    pub(crate) wrappers: Vec<RuleWrapper>,
    pub(crate) pseudos: Vec<String>,
    pub(crate) selector_transform: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum RuleLayer {
    Base,
    Utilities,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) enum RuleWrapper {
    Media(String),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ResolvedAtom {
    pub(crate) selector_class: String,
    pub(crate) variants: Vec<String>,
    pub(crate) base: String,
    pub(crate) layer: RuleLayer,
}

pub(crate) type VariantEffects = (Vec<RuleWrapper>, Vec<String>, Option<String>);
