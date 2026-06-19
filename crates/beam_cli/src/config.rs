use std::{
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};

use beam_core::BeamConfig;

pub(crate) fn default_config_path() -> PathBuf {
    for candidate in [
        "beam.config.ts",
        "beam.config.js",
        "beam.config.mjs",
        "beam.config.json",
    ] {
        let path = PathBuf::from(candidate);
        if path.exists() {
            return path;
        }
    }
    PathBuf::from("beam.config.ts")
}

pub(crate) fn load_config(path: &Path) -> Result<BeamConfig, String> {
    let source =
        fs::read_to_string(path).map_err(|error| format!("failed to read config: {error}"))?;
    let object_source = if path.extension() == Some(OsStr::new("json")) {
        source
    } else {
        extract_config_object(&source)?
    };

    json5::from_str(&object_source).map_err(|error| format!("invalid config: {error}"))
}

pub(crate) fn extract_config_object(source: &str) -> Result<String, String> {
    let start = source
        .find("defineConfig")
        .and_then(|index| source[index..].find('(').map(|open| index + open + 1))
        .or_else(|| {
            source
                .find("export default")
                .map(|index| index + "export default".len())
        })
        .ok_or_else(|| "expected `export default defineConfig({...})`".to_owned())?;

    let mut depth = 0usize;
    let mut object_start = None;
    let mut in_string = None;
    let mut escaped = false;

    for (offset, ch) in source[start..].char_indices() {
        if let Some(quote) = in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == quote {
                in_string = None;
            }
            continue;
        }

        match ch {
            '"' | '\'' | '`' => in_string = Some(ch),
            '{' => {
                if object_start.is_none() {
                    object_start = Some(start + offset);
                }
                depth += 1;
            }
            '}' => {
                depth = depth
                    .checked_sub(1)
                    .ok_or_else(|| "unmatched `}` in config".to_owned())?;
                if depth == 0 {
                    let object_start =
                        object_start.ok_or_else(|| "missing config object".to_owned())?;
                    return Ok(source[object_start..=start + offset].to_owned());
                }
            }
            _ => {}
        }
    }

    Err("missing config object".to_owned())
}
