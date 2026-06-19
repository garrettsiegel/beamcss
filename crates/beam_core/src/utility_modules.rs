use crate::BeamConfig;

pub(crate) fn require_module(config: &BeamConfig, module: &str) -> Result<(), String> {
    if config.utilities.get(module).copied().unwrap_or(true) {
        Ok(())
    } else {
        Err(format!("utility module `{module}` is disabled"))
    }
}
