use std::collections::BTreeMap;

use beam_core::{compile, explain, parse_classlist, BeamConfig, BeamRecipe, BeamTokens};

fn config() -> BeamConfig {
    BeamConfig {
        presets: Vec::new(),
        tokens: BeamTokens {
            spacing: BTreeMap::from([("section".into(), "2rem".into())]),
            color: BTreeMap::from([
                ("accent".into(), "#3b82f6".into()),
                ("base".into(), "#0b0b0c".into()),
                ("fg".into(), "#e8e8ea".into()),
                ("muted".into(), "#6b7280".into()),
                ("on-accent".into(), "#ffffff".into()),
                ("surface".into(), "#16161a".into()),
            ]),
            radius: BTreeMap::from([("md".into(), "8px".into())]),
            text: BTreeMap::from([("base".into(), "16px".into()), ("lg".into(), "20px".into())]),
            font: BTreeMap::from([("ui".into(), "Inter, system-ui, sans-serif".into())]),
            screens: BTreeMap::from([
                ("tablet".into(), "48rem".into()),
                ("desktop".into(), "64rem".into()),
                (
                    "mobile-landscape".into(),
                    "(max-width:47.999rem) and (orientation:landscape)".into(),
                ),
            ]),
        },
        shortcuts: BTreeMap::from([(
            "hero-card".into(),
            "flex direction-column align-center gap-4 p-6 bg-surface rounded-md".into(),
        )]),
        recipes: BTreeMap::from([(
            "button".into(),
            BeamRecipe {
                base: "rounded-md padding:(8 x:12)".into(),
                variants: BTreeMap::from([("primary".into(), "bg-accent text-on-accent".into())]),
            },
        )]),
        utilities: BTreeMap::new(),
        background: Some("base".into()),
        foreground: Some("fg".into()),
    }
}

#[test]
fn representative_markup_matches_golden_css() {
    let result = compile(
        &config(),
        &[
            "grid place-center min-h-screen bg-base text-fg font-ui".into(),
            "hero-card".into(),
            "flex direction-column align-center gap-4 p-6 bg-surface rounded-md hover:(bg-accent text-on-accent scale-105)".into(),
            "tablet:(direction-row justify-between align-center gap-2 text-lg) [&>svg]:(w-[1rem] h-[1rem] text-muted)".into(),
            "padding:(16 top:24) text:(16 bold center) border:(1 solid accent)".into(),
            "button button:primary".into(),
        ],
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(
        result.css,
        include_str!("fixtures/representative-output.css")
    );
}

#[test]
fn parser_reports_edge_case_failures() {
    let cases = [
        ("hover:()", "group has no children"),
        ("hover:(bg-accent", "unclosed `(` in utility"),
        ("hover:bg-accent)", "unmatched `)` in utility"),
        ("md::bg-accent", "empty variant in variant chain"),
        (
            "madeup(bg-accent)",
            "unrecognized group syntax `madeup(...)`",
        ),
    ];

    for (classlist, expected) in cases {
        let errors = parse_classlist(classlist).unwrap_err();
        assert_eq!(errors.len(), 1, "{classlist}");
        assert_eq!(errors[0].message, expected, "{classlist}");
    }
}

#[test]
fn compiler_keeps_going_after_bad_classes() {
    let result = compile(&config(), &["p-4 bogus tablet:unknown text-muted".into()]);

    assert_eq!(result.errors.len(), 2);
    assert!(result.css.contains(".p-4{padding:4px;}"));
    assert!(result
        .css
        .contains(".text-muted{color:var(--color-muted);}"));
    assert_eq!(result.errors[0].class_name, "bogus");
    assert_eq!(result.errors[1].class_name, "tablet:unknown");
}

#[test]
fn explain_reports_flat_grouped_and_dynamic_atoms() {
    let result = explain(
        &config(),
        &[
            "p-4 hover:(bg-accent text-on-accent)".into(),
            "flex align-center gap-(--gap) cols-[200px_1fr]".into(),
        ],
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert_eq!(result.class_strings.len(), 2);

    let first = &result.class_strings[0].tokens;
    assert_eq!(first[0].kind, "utility");
    assert_eq!(first[0].atoms[0].declaration, "padding:4px");
    assert_eq!(first[1].kind, "group");
    assert_eq!(first[1].variants, vec!["hover"]);
    assert_eq!(first[1].atoms.len(), 2);
    assert_eq!(
        first[1].atoms[0].selector,
        ".hover\\:\\(bg-accent.text-on-accent\\):hover"
    );

    let second = &result.class_strings[1].tokens;
    assert_eq!(second[0].kind, "utility");
    assert_eq!(second[0].atoms[0].declaration, "display:flex");
    assert!(second[2]
        .atoms
        .iter()
        .any(|atom| atom.declaration == "gap:var(--gap)"));
    assert!(second[3]
        .atoms
        .iter()
        .any(|atom| atom.declaration == "grid-template-columns:200px 1fr"));
}

#[test]
fn explain_keeps_structured_errors_with_partial_success() {
    let result = explain(&config(), &["p-4 bogus hover:()".into()]);

    assert_eq!(result.errors.len(), 2);
    assert_eq!(result.class_strings[0].tokens.len(), 3);
    assert_eq!(
        result.class_strings[0].tokens[0].atoms[0].declaration,
        "padding:4px"
    );
    assert_eq!(
        result.class_strings[0].tokens[1].errors[0].class_name,
        "bogus"
    );
    assert_eq!(
        result.class_strings[0].tokens[2].errors[0].message,
        "group has no children"
    );
}
