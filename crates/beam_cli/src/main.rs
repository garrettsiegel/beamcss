#![forbid(unsafe_code)]

use std::{
    collections::BTreeMap,
    env,
    ffi::OsStr,
    fs, io,
    path::{Path, PathBuf},
    process, thread,
    time::{Duration, SystemTime},
};

use beam_core::{compile, explain, BeamConfig, CompileMessage, ExplainResult};

const DEFAULT_OUTPUT: &str = "beam.css";
const SUPPORTED_EXTENSIONS: &[&str] = &["html", "jsx", "tsx", "vue", "svelte", "astro"];
const IGNORED_DIRS: &[&str] = &[
    ".git",
    "node_modules",
    "dist",
    "target",
    ".vite",
    ".next",
    "coverage",
];

#[derive(Debug, Clone)]
struct BuildOptions {
    config: PathBuf,
    content: Vec<PathBuf>,
    output: PathBuf,
    watch: bool,
}

#[derive(Debug, Clone)]
struct CheckOptions {
    config: PathBuf,
    content: Vec<PathBuf>,
    format: OutputFormat,
}

#[derive(Debug, Clone)]
struct ExplainOptions {
    config: PathBuf,
    class_string: String,
    format: OutputFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OutputFormat {
    Text,
    Json,
}

fn main() {
    if let Err(error) = run(env::args().skip(1).collect()) {
        eprintln!("beam: {error}");
        process::exit(1);
    }
}

fn run(args: Vec<String>) -> Result<(), String> {
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

fn check(options: CheckOptions) -> Result<(), String> {
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

fn explain_command(options: ExplainOptions) -> Result<(), String> {
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

fn check_text(class_string_count: usize, errors: &[CompileMessage]) -> String {
    if errors.is_empty() {
        return format!("Beam check passed: {class_string_count} class string(s) valid.\n");
    }

    let mut output = format!("Beam check failed: {} error(s).\n", errors.len());
    for error in errors {
        output.push_str(&format!("- {}: {}\n", error.class_name, error.message));
    }
    output
}

fn explain_text(result: &ExplainResult) -> String {
    let mut output = String::new();
    for class_string in &result.class_strings {
        output.push_str(&format!("Class string: {}\n", class_string.class_string));
        for token in &class_string.tokens {
            output.push_str(&format!("- {} ({})\n", token.raw, token.kind));
            for atom in &token.atoms {
                let media = if atom.media.is_empty() {
                    String::new()
                } else {
                    format!(" @media {}", atom.media.join(" and "))
                };
                output.push_str(&format!(
                    "  -> {}{} [{}] {}",
                    atom.selector, media, atom.layer, atom.declaration
                ));
                output.push('\n');
            }
            for error in &token.errors {
                output.push_str(&format!("  ! {}: {}\n", error.class_name, error.message));
            }
        }
    }

    if !result.errors.is_empty() {
        output.push_str(&format!("Errors: {}\n", result.errors.len()));
    }
    output
}

fn default_config_path() -> PathBuf {
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

fn init(args: &[String]) -> Result<(), String> {
    let mut template = InitTemplate::Basic;
    let mut index = 0;

    while index < args.len() {
        match args[index].as_str() {
            "--template" | "-t" => {
                index += 1;
                template = match args.get(index).map(String::as_str) {
                    Some("basic") => InitTemplate::Basic,
                    Some("vite") => InitTemplate::Vite,
                    Some(value) => return Err(format!("unknown init template `{value}`")),
                    None => return Err("--template expects `basic` or `vite`".to_owned()),
                };
            }
            value if value.starts_with('-') => return Err(format!("unknown option `{value}`")),
            value => return Err(format!("unexpected init argument `{value}`")),
        }
        index += 1;
    }

    match template {
        InitTemplate::Basic => init_basic()?,
        InitTemplate::Vite => init_vite()?,
    }
    Ok(())
}

#[derive(Debug, Clone, Copy)]
enum InitTemplate {
    Basic,
    Vite,
}

fn init_basic() -> Result<(), String> {
    write_new_file(Path::new("beam.config.ts"), default_config())?;
    println!("created beam.config.ts");
    Ok(())
}

fn init_vite() -> Result<(), String> {
    write_new_file(Path::new("beam.config.ts"), vite_starter_config())?;
    write_new_file(Path::new("package.json"), vite_package_json())?;
    write_new_file(Path::new("index.html"), vite_index_html())?;
    write_new_file(Path::new("src/main.tsx"), vite_main_tsx())?;
    write_new_file(Path::new("src/App.tsx"), vite_app_tsx())?;
    write_new_file(Path::new("vite.config.ts"), vite_config_ts())?;
    println!("created Beam Vite starter");
    Ok(())
}

fn write_new_file(path: &Path, contents: &str) -> Result<(), String> {
    if path.exists() {
        return Err(format!("{} already exists", path.display()));
    }

    if let Some(parent) = path
        .parent()
        .filter(|parent| !parent.as_os_str().is_empty())
    {
        fs::create_dir_all(parent)
            .map_err(|error| format!("failed to create {}: {error}", parent.display()))?;
    }

    fs::write(path, contents)
        .map_err(|error| format!("failed to write {}: {error}", path.display()))
}

fn default_config() -> &'static str {
    r##"import { defineConfig } from "beamcss"

export default defineConfig({
  tokens: {
    // Beam ships no color palette — define your own tokens here, then use them
    // as bg-<name>, fg-<name>, bd-<name>. Modify inline with the color algebra:
    //   bg-brand/50  (alpha)   bg-brand+10 / bg-brand-10  (lighten / darken)
    //   bg-brand~ink (mix two tokens)
    color: {},
    space: { card: "1rem", section: "2rem" },
    radius: { sm: "4px", md: "8px", lg: "16px", full: "9999px" },
    text: { sm: "14px", base: "16px", lg: "20px", xl: "28px" },
    font: { ui: "Inter, system-ui, sans-serif", mono: "ui-monospace, monospace" },
    screens: {
      tablet: "48rem",
      desktop: "64rem",
      wide: "80rem",
      "mobile-landscape": "(max-width:47.999rem) and (orientation:landscape)",
    },
  },
  // Point these at two of your color tokens to paint the page <body>:
  // background: "page",
  // foreground: "ink",
})
"##
}

fn vite_starter_config() -> &'static str {
    r##"import { defineConfig } from "beamcss"

export default defineConfig({
  tokens: {
    color: {
      page: "#0b0b0c",
      surface: "#16161a",
      ink: "#e8e8ea",
      muted: "#6b7280",
      brand: "#3b82f6",
      "on-brand": "#ffffff",
    },
    space: { card: "1rem", section: "2rem" },
    radius: { sm: "4px", md: "8px", lg: "16px", full: "9999px" },
    text: { sm: "14px", base: "16px", lg: "20px", xl: "28px" },
    font: { ui: "Inter, system-ui, sans-serif", mono: "ui-monospace, monospace" },
    screens: {
      tablet: "48rem",
      desktop: "64rem",
      wide: "80rem",
      "mobile-landscape": "(max-width:47.999rem) and (orientation:landscape)",
    },
  },
  background: "page",
  foreground: "ink",
})
"##
}

fn vite_package_json() -> &'static str {
    r#"{
  "name": "beamcss-app",
  "private": true,
  "type": "module",
  "scripts": {
    "dev": "vite",
    "build": "vite build",
    "preview": "vite preview"
  },
  "dependencies": {
    "@beamcss/vite": "latest",
    "@vitejs/plugin-react": "latest",
    "beamcss": "latest",
    "react": "latest",
    "react-dom": "latest",
    "vite": "latest"
  },
  "devDependencies": {
    "@types/react": "latest",
    "@types/react-dom": "latest",
    "typescript": "latest"
  }
}
"#
}

fn vite_index_html() -> &'static str {
    r#"<div id="root"></div>
<script type="module" src="/src/main.tsx"></script>
"#
}

fn vite_main_tsx() -> &'static str {
    r#"import { StrictMode } from "react"
import { createRoot } from "react-dom/client"
import App from "./App"

createRoot(document.getElementById("root")!).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
"#
}

fn vite_app_tsx() -> &'static str {
    r#"export default function App() {
  return (
    <main className="place min-h-screen bg-page fg-ink font-ui">
      <section className="stack(center gap-4) p-6 bg-surface round-lg hover:(bg-surface+8 scale-105)">
        <p className="text-sm fg-brand">beamcss</p>
        <h1 className="text-xl">Focused styles, zero scatter.</h1>
        <p className="text-base fg-muted max-w-[36rem]">
          Edit <code>src/App.tsx</code> and write Beam classes inline.
        </p>
      </section>
    </main>
  )
}
"#
}

fn vite_config_ts() -> &'static str {
    r#"import { defineConfig } from "vite"
import react from "@vitejs/plugin-react"
import { beamcss } from "@beamcss/vite"

export default defineConfig({
  plugins: [
    react(),
    beamcss({
      config: "beam.config.ts",
      content: ["index.html", "src"],
    }),
  ],
})
"#
}

fn build(options: BuildOptions) -> Result<(), String> {
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

fn load_config(path: &Path) -> Result<BeamConfig, String> {
    let source =
        fs::read_to_string(path).map_err(|error| format!("failed to read config: {error}"))?;
    let object_source = if path.extension() == Some(OsStr::new("json")) {
        source
    } else {
        extract_config_object(&source)?
    };

    json5::from_str(&object_source).map_err(|error| format!("invalid config: {error}"))
}

fn extract_config_object(source: &str) -> Result<String, String> {
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

fn collect_files(paths: &[PathBuf]) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    for path in paths {
        collect_path(path, &mut files)?;
    }
    files.sort();
    files.dedup();
    Ok(files)
}

fn collect_path(path: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    if path.is_file() {
        if is_supported_source(path) {
            files.push(path.to_owned());
        }
        return Ok(());
    }

    if !path.is_dir() || ignored_dir(path) {
        return Ok(());
    }

    for entry in fs::read_dir(path)? {
        collect_path(&entry?.path(), files)?;
    }
    Ok(())
}

fn ignored_dir(path: &Path) -> bool {
    path.file_name()
        .and_then(OsStr::to_str)
        .is_some_and(|name| IGNORED_DIRS.contains(&name))
}

fn is_supported_source(path: &Path) -> bool {
    path.extension()
        .and_then(OsStr::to_str)
        .is_some_and(|extension| SUPPORTED_EXTENSIONS.contains(&extension))
}

fn scan_files(files: &[PathBuf]) -> io::Result<Vec<String>> {
    let mut classes = Vec::new();
    for file in files {
        let source = fs::read_to_string(file)?;
        classes.extend(extract_class_strings(&source));
    }
    Ok(classes)
}

fn extract_class_strings(source: &str) -> Vec<String> {
    let mut classes = Vec::new();
    for attribute in ["class", "className"] {
        let mut offset = 0usize;
        while let Some(index) = source[offset..].find(attribute) {
            let start = offset + index;
            if !attribute_boundary(source, start, attribute.len()) {
                offset = start + attribute.len();
                continue;
            }
            if let Some((class_string, next)) =
                read_attribute_value(source, start + attribute.len())
            {
                classes.push(class_string);
                offset = next;
            } else {
                offset = start + attribute.len();
            }
        }
    }
    classes
}

fn attribute_boundary(source: &str, start: usize, len: usize) -> bool {
    let before = source[..start].chars().next_back();
    let after = source[start + len..].chars().next();
    !before.is_some_and(is_ident_char) && !after.is_some_and(is_ident_char)
}

fn is_ident_char(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_' || ch == '-'
}

fn read_attribute_value(source: &str, mut index: usize) -> Option<(String, usize)> {
    index = skip_ws(source, index);
    if source[index..].chars().next()? != '=' {
        return None;
    }
    index += 1;
    index = skip_ws(source, index);

    let close_brace = if source[index..].chars().next() == Some('{') {
        index += 1;
        index = skip_ws(source, index);
        true
    } else {
        false
    };

    let quote = source[index..].chars().next()?;
    if quote != '"' && quote != '\'' && quote != '`' {
        return None;
    }
    index += quote.len_utf8();
    let value_start = index;
    let mut escaped = false;

    while index < source.len() {
        let ch = source[index..].chars().next()?;
        if escaped {
            escaped = false;
        } else if ch == '\\' {
            escaped = true;
        } else if ch == quote {
            let value = source[value_start..index].to_owned();
            index += ch.len_utf8();
            if close_brace {
                index = skip_ws(source, index);
                if source[index..].chars().next() == Some('}') {
                    index += 1;
                }
            }
            return Some((value, index));
        }
        index += ch.len_utf8();
    }

    None
}

fn skip_ws(source: &str, mut index: usize) -> usize {
    while index < source.len() {
        let Some(ch) = source[index..].chars().next() else {
            break;
        };
        if !ch.is_whitespace() {
            break;
        }
        index += ch.len_utf8();
    }
    index
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Mutex, OnceLock};
    use std::time::{SystemTime, UNIX_EPOCH};

    fn cwd_lock() -> &'static Mutex<()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(()))
    }

    #[test]
    fn extracts_plain_and_jsx_class_strings() {
        let source = r#"
          <main class="p-4 bg-surface">
          <div className={'stack(gap-4)'}>
          <span className={`fg-accent`}>
        "#;

        assert_eq!(
            extract_class_strings(source),
            vec!["p-4 bg-surface", "stack(gap-4)", "fg-accent"]
        );
    }

    #[test]
    fn extracts_define_config_object() {
        let source = r#"
          import { defineConfig } from "beamcss"
          export default defineConfig({
            tokens: { space: { card: "1rem" } },
          })
        "#;

        assert_eq!(
            extract_config_object(source).unwrap(),
            "{\n            tokens: { space: { card: \"1rem\" } },\n          }"
        );
    }

    #[test]
    fn init_vite_creates_starter_files() {
        let _guard = cwd_lock().lock().unwrap();
        let original = env::current_dir().unwrap();
        let temp = env::temp_dir().join(format!(
            "beam-init-vite-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&temp).unwrap();
        env::set_current_dir(&temp).unwrap();

        let result = run(vec![
            "init".to_owned(),
            "--template".to_owned(),
            "vite".to_owned(),
        ]);

        env::set_current_dir(original).unwrap();

        assert!(result.is_ok(), "{result:?}");
        assert!(temp.join("beam.config.ts").exists());
        assert!(temp.join("package.json").exists());
        assert!(temp.join("src/App.tsx").exists());
        assert!(fs::read_to_string(temp.join("vite.config.ts"))
            .unwrap()
            .contains("@beamcss/vite"));
        fs::remove_dir_all(temp).unwrap();
    }

    #[test]
    fn init_refuses_to_overwrite_existing_config() {
        let _guard = cwd_lock().lock().unwrap();
        let original = env::current_dir().unwrap();
        let temp = env::temp_dir().join(format!(
            "beam-init-existing-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(&temp).unwrap();
        fs::write(temp.join("beam.config.ts"), "already here").unwrap();
        env::set_current_dir(&temp).unwrap();

        let result = run(vec!["init".to_owned()]);

        env::set_current_dir(original).unwrap();

        assert_eq!(result.unwrap_err(), "beam.config.ts already exists");
        fs::remove_dir_all(temp).unwrap();
    }

    #[test]
    fn check_detects_invalid_classes_in_scanned_files() {
        let _guard = cwd_lock().lock().unwrap();
        let original = env::current_dir().unwrap();
        let temp = env::temp_dir().join(format!(
            "beam-check-invalid-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        fs::create_dir_all(temp.join("src")).unwrap();
        fs::write(temp.join("beam.config.ts"), default_config()).unwrap();
        fs::write(
            temp.join("src/App.tsx"),
            r#"<main className="p-4 bogus hover:()"></main>"#,
        )
        .unwrap();
        env::set_current_dir(&temp).unwrap();

        let result = run(vec![
            "check".to_owned(),
            "--config".to_owned(),
            "beam.config.ts".to_owned(),
            "--content".to_owned(),
            "src".to_owned(),
        ]);

        env::set_current_dir(original).unwrap();

        assert_eq!(result.unwrap_err(), "check failed");
        fs::remove_dir_all(temp).unwrap();
    }

    #[test]
    fn explain_text_summarizes_atoms_and_errors() {
        let config_source = r##"export default defineConfig({
  tokens: { color: { accent: "#3b82f6" } },
})"##;
        let result = explain(
            &json5::from_str(&extract_config_object(config_source).unwrap()).unwrap(),
            &["p-4 hover:(bg-accent bogus)".to_owned()],
        );
        let output = explain_text(&result);

        assert!(output.contains("Class string: p-4 hover:(bg-accent bogus)"));
        assert!(output.contains("-> .p-4 [beam.utilities] padding:4px"));
        assert!(output.contains("! hover:bogus: unsupported utility `bogus`"));
        assert!(output.contains("Errors: 1"));
    }

    #[test]
    fn check_text_is_concise_for_agents_and_humans() {
        let errors = vec![CompileMessage {
            class_name: "bogus".to_owned(),
            message: "unsupported utility `bogus`".to_owned(),
        }];

        assert_eq!(
            check_text(2, &[]),
            "Beam check passed: 2 class string(s) valid.\n"
        );
        assert_eq!(
            check_text(2, &errors),
            "Beam check failed: 1 error(s).\n- bogus: unsupported utility `bogus`\n"
        );
    }
}
