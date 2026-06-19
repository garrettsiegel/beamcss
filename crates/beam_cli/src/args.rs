use std::path::PathBuf;

use crate::{
    commands::{build, check, explain_command},
    config::default_config_path,
    init::init,
};

const DEFAULT_OUTPUT: &str = "beam.css";

#[derive(Debug, Clone)]
pub(crate) struct BuildOptions {
    pub(crate) config: PathBuf,
    pub(crate) content: Vec<PathBuf>,
    pub(crate) output: PathBuf,
    pub(crate) watch: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct CheckOptions {
    pub(crate) config: PathBuf,
    pub(crate) content: Vec<PathBuf>,
    pub(crate) format: OutputFormat,
}

#[derive(Debug, Clone)]
pub(crate) struct ExplainOptions {
    pub(crate) config: PathBuf,
    pub(crate) class_string: String,
    pub(crate) format: OutputFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum OutputFormat {
    Text,
    Json,
}

pub(crate) fn run(args: Vec<String>) -> Result<(), String> {
    match args.first().map(String::as_str) {
        None | Some("--help" | "-h") => {
            print_help();
            Ok(())
        }
        Some("build") => build(parse_build_options(&args[1..], false)?),
        Some("check") => check(parse_check_options(&args[1..])?),
        Some("explain") => explain_command(parse_explain_options(&args[1..])?),
        Some("init") => init(&args[1..]),
        Some("dev") => {
            let options = parse_build_options(&args[1..], true)?;
            build(options)
        }
        Some(command) => Err(format!("unknown command `{command}`")),
    }
}

fn print_help() {
    println!("beamcss");
    println!();
    println!("Commands:");
    println!("  beam init [--template basic|vite]");
    println!("  beam build [--config beam.config.ts] [--content .] [--out beam.css]");
    println!("  beam check [--config beam.config.ts] [--content .] [--format text|json]");
    println!("  beam explain \"<class string>\" [--config beam.config.ts] [--format text|json]");
    println!("  beam dev   [--config beam.config.ts] [--content .] [--out beam.css]");
}

fn parse_build_options(args: &[String], watch: bool) -> Result<BuildOptions, String> {
    let mut config = default_config_path();
    let mut content = Vec::new();
    let mut output = PathBuf::from(DEFAULT_OUTPUT);
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--config" | "-c" => {
                index += 1;
                config = next_path(args, index, "--config")?;
            }
            "--content" => {
                index += 1;
                content.push(next_path(args, index, "--content")?);
            }
            "--out" | "-o" => {
                index += 1;
                output = next_path(args, index, "--out")?;
            }
            value if value.starts_with('-') => return Err(format!("unknown option `{value}`")),
            value => content.push(PathBuf::from(value)),
        }
        index += 1;
    }

    if content.is_empty() {
        content.push(PathBuf::from("."));
    }

    Ok(BuildOptions {
        config,
        content,
        output,
        watch,
    })
}

fn parse_check_options(args: &[String]) -> Result<CheckOptions, String> {
    let mut config = default_config_path();
    let mut content = Vec::new();
    let mut format = OutputFormat::Text;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--config" | "-c" => {
                index += 1;
                config = next_path(args, index, "--config")?;
            }
            "--content" => {
                index += 1;
                content.push(next_path(args, index, "--content")?);
            }
            "--format" => {
                index += 1;
                format = parse_format(args.get(index).map(String::as_str))?;
            }
            value if value.starts_with('-') => return Err(format!("unknown option `{value}`")),
            value => content.push(PathBuf::from(value)),
        }
        index += 1;
    }

    if content.is_empty() {
        content.push(PathBuf::from("."));
    }

    Ok(CheckOptions {
        config,
        content,
        format,
    })
}

fn parse_explain_options(args: &[String]) -> Result<ExplainOptions, String> {
    let mut config = default_config_path();
    let mut class_string = None;
    let mut format = OutputFormat::Text;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--config" | "-c" => {
                index += 1;
                config = next_path(args, index, "--config")?;
            }
            "--format" => {
                index += 1;
                format = parse_format(args.get(index).map(String::as_str))?;
            }
            value if value.starts_with('-') => return Err(format!("unknown option `{value}`")),
            value => {
                if class_string.is_some() {
                    return Err(format!("unexpected explain argument `{value}`"));
                }
                class_string = Some(value.to_owned());
            }
        }
        index += 1;
    }

    Ok(ExplainOptions {
        config,
        class_string: class_string.ok_or_else(|| "explain expects a class string".to_owned())?,
        format,
    })
}

fn parse_format(value: Option<&str>) -> Result<OutputFormat, String> {
    match value {
        Some("text") => Ok(OutputFormat::Text),
        Some("json") => Ok(OutputFormat::Json),
        Some(value) => Err(format!("unknown output format `{value}`")),
        None => Err("--format expects `text` or `json`".to_owned()),
    }
}

fn next_path(args: &[String], index: usize, option: &str) -> Result<PathBuf, String> {
    args.get(index)
        .map(PathBuf::from)
        .ok_or_else(|| format!("{option} expects a path"))
}
