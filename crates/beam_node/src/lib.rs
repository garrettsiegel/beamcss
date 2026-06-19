#![forbid(unsafe_code)]

use beam_core::{compile as compile_core, explain as explain_core, BeamConfig};
use napi_derive::napi;

#[napi]
pub fn compile(config_json: String, class_strings: Vec<String>) -> napi::Result<String> {
    let config: BeamConfig = serde_json::from_str(&config_json)
        .map_err(|error| napi::Error::from_reason(format!("invalid Beam config JSON: {error}")))?;
    let result = compile_core(&config, &class_strings);

    serde_json::to_string(&result)
        .map_err(|error| napi::Error::from_reason(format!("failed to serialize result: {error}")))
}

#[napi]
pub fn explain(config_json: String, class_strings: Vec<String>) -> napi::Result<String> {
    let config: BeamConfig = serde_json::from_str(&config_json)
        .map_err(|error| napi::Error::from_reason(format!("invalid Beam config JSON: {error}")))?;
    let result = explain_core(&config, &class_strings);

    serde_json::to_string(&result)
        .map_err(|error| napi::Error::from_reason(format!("failed to serialize result: {error}")))
}
