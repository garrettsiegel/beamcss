use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct BeamConfig {
    #[serde(default)]
    pub presets: Vec<BeamPreset>,
    pub tokens: BeamTokens,
    #[serde(default)]
    pub shortcuts: BTreeMap<String, String>,
    #[serde(default)]
    pub recipes: BTreeMap<String, BeamRecipe>,
    #[serde(default)]
    pub utilities: BTreeMap<String, bool>,
    /// Color token the reset paints onto `body`'s background. Names one of
    /// `tokens.color`; there are no blessed defaults.
    #[serde(default)]
    pub background: Option<String>,
    /// Color token the reset uses for `body`'s text color.
    #[serde(default)]
    pub foreground: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct BeamPreset {
    #[serde(default)]
    pub tokens: BeamTokens,
    #[serde(default)]
    pub shortcuts: BTreeMap<String, String>,
    #[serde(default)]
    pub recipes: BTreeMap<String, BeamRecipe>,
    #[serde(default)]
    pub utilities: BTreeMap<String, bool>,
    #[serde(default)]
    pub background: Option<String>,
    #[serde(default)]
    pub foreground: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct BeamRecipe {
    #[serde(default)]
    pub base: String,
    #[serde(default)]
    pub variants: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize, PartialEq, Eq)]
pub struct BeamTokens {
    #[serde(default, alias = "space")]
    pub spacing: BTreeMap<String, String>,
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

pub(crate) fn resolve_presets(config: &BeamConfig) -> BeamConfig {
    let mut resolved = BeamConfig::default();

    for preset in &config.presets {
        merge_preset(&mut resolved, preset);
    }

    resolved
        .tokens
        .spacing
        .extend(config.tokens.spacing.clone());
    resolved.tokens.color.extend(config.tokens.color.clone());
    resolved.tokens.radius.extend(config.tokens.radius.clone());
    resolved.tokens.text.extend(config.tokens.text.clone());
    resolved.tokens.font.extend(config.tokens.font.clone());
    resolved
        .tokens
        .screens
        .extend(config.tokens.screens.clone());
    resolved.shortcuts.extend(config.shortcuts.clone());
    resolved.recipes.extend(config.recipes.clone());
    resolved.utilities.extend(config.utilities.clone());
    resolved.background = config.background.clone().or(resolved.background);
    resolved.foreground = config.foreground.clone().or(resolved.foreground);

    resolved
}

fn merge_preset(config: &mut BeamConfig, preset: &BeamPreset) {
    config.tokens.spacing.extend(preset.tokens.spacing.clone());
    config.tokens.color.extend(preset.tokens.color.clone());
    config.tokens.radius.extend(preset.tokens.radius.clone());
    config.tokens.text.extend(preset.tokens.text.clone());
    config.tokens.font.extend(preset.tokens.font.clone());
    config.tokens.screens.extend(preset.tokens.screens.clone());
    config.shortcuts.extend(preset.shortcuts.clone());
    config.recipes.extend(preset.recipes.clone());
    config.utilities.extend(preset.utilities.clone());
    config.background = preset.background.clone().or(config.background.clone());
    config.foreground = preset.foreground.clone().or(config.foreground.clone());
}
