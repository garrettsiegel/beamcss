use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};
use std::{env, fs};

use beam_core::{explain, CompileMessage};

use crate::{
    args::run,
    config::extract_config_object,
    init::default_config,
    output::{check_text, explain_text},
    scanner::extract_class_strings,
};

fn cwd_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

#[test]
fn extracts_plain_and_jsx_class_strings() {
    let source = r#"
      <main class="p-4 bg-surface">
      <div className={'flex direction-column gap-4'}>
      <span className={`text-accent`}>
    "#;

    assert_eq!(
        extract_class_strings(source),
        vec![
            "p-4 bg-surface",
            "flex direction-column gap-4",
            "text-accent"
        ]
    );
}

#[test]
fn extracts_define_config_object() {
    let source = r#"
      import { defineConfig } from "beamcss"
      export default defineConfig({
        tokens: { spacing: { card: "1rem" } },
      })
    "#;

    assert_eq!(
        extract_config_object(source).unwrap(),
        "{\n        tokens: { spacing: { card: \"1rem\" } },\n      }"
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
