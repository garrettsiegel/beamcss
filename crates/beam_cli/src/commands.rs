use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    thread,
    time::{Duration, SystemTime},
};

use beam_core::{compile, explain};

use crate::{
    args::{BuildOptions, CheckOptions, ExplainOptions, OutputFormat},
    config::load_config,
    output::{check_text, explain_text},
    scanner::{collect_files, scan_files},
};

pub(crate) fn check(options: CheckOptions) -> Result<(), String> {
    let config = load_config(&options.config)?;
    let files = collect_files(&options.content).map_err(|error| format!("scan failed: {error}"))?;
    let class_strings = scan_files(&files).map_err(|error| format!("scan failed: {error}"))?;
    let result = compile(&config, &class_strings);
    let failed = !result.errors.is_empty();

    match options.format {
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&serde_json::json!({
                    "valid": !failed,
                    "class_string_count": class_strings.len(),
                    "warnings": result.warnings,
                    "errors": result.errors,
                }))
                .map_err(|error| format!("failed to serialize check result: {error}"))?
            );
        }
        OutputFormat::Text => print!("{}", check_text(class_strings.len(), &result.errors)),
    }

    if failed {
        Err("check failed".to_owned())
    } else {
        Ok(())
    }
}

pub(crate) fn explain_command(options: ExplainOptions) -> Result<(), String> {
    let config = load_config(&options.config)?;
    let result = explain(&config, std::slice::from_ref(&options.class_string));
    let failed = !result.errors.is_empty();

    match options.format {
        OutputFormat::Json => {
            println!(
                "{}",
                serde_json::to_string_pretty(&result)
                    .map_err(|error| format!("failed to serialize explain result: {error}"))?
            );
        }
        OutputFormat::Text => print!("{}", explain_text(&result)),
    }

    if failed {
        Err("explain failed".to_owned())
    } else {
        Ok(())
    }
}

pub(crate) fn build(options: BuildOptions) -> Result<(), String> {
    if options.watch {
        return watch(options);
    }

    build_once(&options)
}

fn watch(options: BuildOptions) -> Result<(), String> {
    build_once(&options)?;
    println!("watching for changes...");

    let mut previous = file_snapshot(&options)?;

    loop {
        thread::sleep(Duration::from_millis(500));
        let current = file_snapshot(&options)?;
        if current == previous {
            continue;
        }

        previous = current;
        match build_once(&options) {
            Ok(()) => {}
            Err(error) => eprintln!("beam: {error}"),
        }
    }
}

fn build_once(options: &BuildOptions) -> Result<(), String> {
    let config = load_config(&options.config)?;
    let files = collect_files(&options.content).map_err(|error| format!("scan failed: {error}"))?;
    let class_strings = scan_files(&files).map_err(|error| format!("scan failed: {error}"))?;
    let result = compile(&config, &class_strings);

    if !result.errors.is_empty() {
        for error in result.errors {
            eprintln!("{}: {}", error.class_name, error.message);
        }
        return Err("compile failed".to_owned());
    }

    if let Some(parent) = options
        .output
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create output directory: {error}"))?;
    }
    fs::write(&options.output, result.css)
        .map_err(|error| format!("failed to write {}: {error}", options.output.display()))?;
    println!("wrote {}", options.output.display());
    Ok(())
}

fn file_snapshot(options: &BuildOptions) -> Result<BTreeMap<PathBuf, Option<SystemTime>>, String> {
    let mut snapshot = BTreeMap::new();
    snapshot.insert(options.config.clone(), modified_time(&options.config));
    for file in collect_files(&options.content).map_err(|error| format!("scan failed: {error}"))? {
        snapshot.insert(file.clone(), modified_time(&file));
    }
    Ok(snapshot)
}

fn modified_time(path: &Path) -> Option<SystemTime> {
    fs::metadata(path)
        .and_then(|metadata| metadata.modified())
        .ok()
}
