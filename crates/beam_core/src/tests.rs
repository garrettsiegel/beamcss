use std::collections::BTreeMap;

use crate::*;

mod colors;

fn config() -> BeamConfig {
    BeamConfig {
        presets: Vec::new(),
        tokens: BeamTokens {
            spacing: BTreeMap::from([("card".into(), "1.25rem".into())]),
            color: BTreeMap::from([
                ("accent".into(), "#3b82f6".into()),
                ("on-accent".into(), "#ffffff".into()),
                ("surface".into(), "#16161a".into()),
            ]),
            radius: BTreeMap::from([("md".into(), "8px".into())]),
            text: BTreeMap::from([("lg".into(), "20px".into())]),
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
        shortcuts: BTreeMap::new(),
        recipes: BTreeMap::new(),
        utilities: BTreeMap::new(),
        background: Some("base".into()),
        foreground: Some("fg".into()),
    }
}

#[test]
fn emits_tokens_and_utilities() {
    let result = compile(
        &config(),
        &["p-1 px-2 gap-card bg-surface text-accent rounded-md text-lg".into()],
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert!(result.css.contains("@layer beam.reset{"));
    assert!(result.css.contains("body{min-width:320px;min-height:100vh;margin:0;background:var(--color-base);color:var(--color-fg);}"));
    assert!(result.css.contains("--space-card:1.25rem;"));
    assert!(result
        .css
        .contains(".bg-surface{background:var(--color-surface);}"));
    assert!(result
        .css
        .contains(".text-accent{color:var(--color-accent);}"));
    assert!(result.css.contains(".gap-card{gap:var(--space-card);}"));
    assert!(result.css.contains(".p-1{padding:1px;}"));
    assert!(result.css.contains(".px-2{padding-inline:2px;}"));
    assert!(result
        .css
        .contains(".rounded-md{border-radius:var(--radius-md);}"));
    assert!(result.css.contains(".text-lg{font-size:var(--text-lg);}"));
}

#[test]
fn reset_paints_body_from_config_pointers() {
    let mut cfg = config();
    cfg.tokens.color.insert("page".into(), "#000".into());
    cfg.background = Some("page".into());
    cfg.foreground = Some("surface".into());
    let result = compile(&cfg, &[]);
    assert!(result.css.contains(
            "body{min-width:320px;min-height:100vh;margin:0;background:var(--color-page);color:var(--color-surface);}"
        ));
}

#[test]
fn reset_omits_body_color_when_pointers_unset() {
    let mut cfg = config();
    cfg.background = None;
    cfg.foreground = None;
    let result = compile(&cfg, &[]);
    assert!(result
        .css
        .contains("body{min-width:320px;min-height:100vh;margin:0;}"));
}

#[test]
fn dedupes_repeated_atoms() {
    let result = compile(&config(), &["p-1 p-1".into(), "p-1".into()]);

    assert!(result.errors.is_empty());
    assert_eq!(result.css.matches(".p-1{").count(), 1);
}

#[test]
fn unfolds_grouped_and_nested_variants() {
    let result = compile(
        &config(),
        &["tablet:(p-6 rounded-md hover:(bg-surface text-on-accent))".into()],
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert!(result.css.contains(
            "@media (min-width:48rem){.tablet\\:\\(p-6.rounded-md.hover\\:\\(bg-surface.text-on-accent\\)\\):hover{background:var(--color-surface);}}\n"
        ));
    assert!(result.css.contains(
            "@media (min-width:48rem){.tablet\\:\\(p-6.rounded-md.hover\\:\\(bg-surface.text-on-accent\\)\\):hover{color:var(--color-on-accent);}}\n"
        ));
}

#[test]
fn unfolds_utility_groups() {
    let result = compile(
        &config(),
        &["padding:(16 top:24) text:(16 bold center) border:(1 solid accent)".into()],
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert!(result.css.contains("padding:16px;"));
    assert!(result.css.contains("padding-top:24px;"));
    assert!(result.css.contains("font-size:16px;"));
    assert!(result.css.contains("font-weight:700;"));
    assert!(result.css.contains("text-align:center;"));
    assert!(result.css.contains("border-width:1px;border-style:solid;"));
    assert!(result.css.contains("border-style:solid;"));
    assert!(result.css.contains("border-color:var(--color-accent);"));
}

#[test]
fn expands_configured_shortcuts() {
    let mut cfg = config();
    cfg.shortcuts.insert(
        "card".into(),
        "padding:(16 top:24) text:(16 bold center) bg-surface".into(),
    );

    let result = compile(&cfg, &["hover:card".into()]);

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert!(result.css.contains(".hover\\:card:hover{padding:16px;}"));
    assert!(result
        .css
        .contains(".hover\\:card:hover{padding-top:24px;}"));
    assert!(result.css.contains(".hover\\:card:hover{font-weight:700;}"));
    assert!(result
        .css
        .contains(".hover\\:card:hover{background:var(--color-surface);}"));
}

#[test]
fn reports_recursive_shortcuts() {
    let mut cfg = config();
    cfg.shortcuts.insert("loop".into(), "loop".into());

    let result = compile(&cfg, &["loop".into()]);

    assert_eq!(result.errors.len(), 1);
    assert_eq!(result.errors[0].class_name, "loop");
    assert_eq!(
        result.errors[0].message,
        "shortcut `loop` expands recursively"
    );
}

#[test]
fn expands_configured_recipes() {
    let mut cfg = config();
    cfg.recipes.insert(
        "button".into(),
        BeamRecipe {
            base: "rounded-md padding:(8 x:12)".into(),
            variants: BTreeMap::from([("primary".into(), "bg-accent text-on-accent".into())]),
        },
    );

    let result = compile(&cfg, &["button hover:button:primary".into()]);

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert!(result
        .css
        .contains(".button{border-radius:var(--radius-md);}"));
    assert!(result.css.contains(".button{padding:8px;}"));
    assert!(result.css.contains(".button{padding-inline:12px;}"));
    assert!(result
        .css
        .contains(".hover\\:button\\:primary:hover{background:var(--color-accent);}"));
    assert!(result
        .css
        .contains(".hover\\:button\\:primary:hover{color:var(--color-on-accent);}"));
}

#[test]
fn utility_modules_can_disable_families() {
    let mut cfg = config();
    cfg.utilities.insert("colors".into(), false);
    cfg.utilities.insert("spacing".into(), false);

    let result = compile(&cfg, &["bg-surface p-4 flex text-lg".into()]);

    assert_eq!(result.errors.len(), 2);
    assert_eq!(result.errors[0].class_name, "bg-surface");
    assert_eq!(
        result.errors[0].message,
        "utility module `colors` is disabled"
    );
    assert_eq!(result.errors[1].class_name, "p-4");
    assert_eq!(
        result.errors[1].message,
        "utility module `spacing` is disabled"
    );
    assert!(result.css.contains(".flex{display:flex;}"));
    assert!(result.css.contains(".text-lg{font-size:var(--text-lg);}"));
}

#[test]
fn presets_merge_before_local_config() {
    let mut cfg = config();
    cfg.presets.push(BeamPreset {
        tokens: BeamTokens {
            spacing: BTreeMap::from([("preset".into(), "2rem".into())]),
            color: BTreeMap::from([
                ("accent".into(), "#000000".into()),
                ("preset".into(), "#111111".into()),
            ]),
            ..BeamTokens::default()
        },
        shortcuts: BTreeMap::from([("center".into(), "grid place-center".into())]),
        ..BeamPreset::default()
    });
    cfg.tokens.color.insert("accent".into(), "#3b82f6".into());

    let result = compile(&cfg, &["center p-preset bg-preset bg-accent".into()]);

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert!(result.css.contains("--space-preset:2rem;"));
    assert!(result.css.contains("--color-preset:#111111;"));
    assert!(result.css.contains("--color-accent:#3b82f6;"));
    assert!(result.css.contains(".center{display:grid;}"));
    assert!(result.css.contains(".center{place-items:center;}"));
    assert!(result
        .css
        .contains(".p-preset{padding:var(--space-preset);}"));
    assert!(result
        .css
        .contains(".bg-preset{background:var(--color-preset);}"));
}

#[test]
fn compiles_arbitrary_and_dynamic_values() {
    let result = compile(
            &config(),
            &[
                "w-[347px] cols-[200px_1fr] w-(--w) w-screen h-screen bg-[red] p-0 p-[1rem] gap-(--gap) scale-105"
                    .into(),
            ],
        );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert!(result.css.contains(".w-\\[347px\\]{width:347px;}"));
    assert!(result
        .css
        .contains(".cols-\\[200px_1fr\\]{grid-template-columns:200px 1fr;}"));
    assert!(result.css.contains(".w-\\(--w\\){width:var(--w);}"));
    assert!(result.css.contains(".w-screen{width:100vw;}"));
    assert!(result.css.contains(".h-screen{height:100vh;}"));
    assert!(result.css.contains(".bg-\\[red\\]{background:red;}"));
    assert!(result.css.contains(".p-0{padding:0;}"));
    assert!(result.css.contains(".p-\\[1rem\\]{padding:1rem;}"));
    assert!(result.css.contains(".gap-\\(--gap\\){gap:var(--gap);}"));
    assert!(result.css.contains(".scale-105{transform:scale(1.05);}"));
}

#[test]
fn compiles_modern_color_syntaxes() {
    let result = compile(
            &config(),
            &[
                "bg-[#fff] text-[rgb(255_255_255_/_80%)] bg-[hsl(220_80%_56%)] text-[oklch(72%_0.14_240)] bg-[color(display-p3_0.2_0.7_0.5)] border-[color-mix(in_srgb,var(--color-surface),white_8%)]".into(),
            ],
        );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert!(result.css.contains(".bg-\\[\\#fff\\]{background:#fff;}"));
    assert!(result
        .css
        .contains(".text-\\[rgb\\(255_255_255_\\/_80\\%\\)\\]{color:rgb(255 255 255 / 80%);}"));
    assert!(result
        .css
        .contains(".text-\\[oklch\\(72\\%_0\\.14_240\\)\\]{color:oklch(72% 0.14 240);}"));
    assert!(result
        .css
        .contains("background:color(display-p3 0.2 0.7 0.5);"));
    assert!(result
        .css
        .contains("border-color:color-mix(in srgb,var(--color-surface),white 8%);"));
}

#[test]
fn compiles_named_and_full_media_queries() {
    let result = compile(
        &config(),
        &["tablet:p-4 desktop:p-8 mobile-landscape:gap-4".into()],
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert!(result
        .css
        .contains("@media (min-width:48rem){.tablet\\:p-4{padding:4px;}}"));
    assert!(result
        .css
        .contains("@media (min-width:64rem){.desktop\\:p-8{padding:8px;}}"));
    assert!(result.css.contains(
            "@media (max-width:47.999rem) and (orientation:landscape){.mobile-landscape\\:gap-4{gap:4px;}}"
        ));
}

#[test]
fn compiles_homepage_utility_surface() {
    let result = compile(
            &config(),
            &[
                "relative absolute overflow-hidden overflow-x-auto z-[1] inset-x-[0] top-[3rem] h-[12rem]".into(),
                "leading-tight tracking-widest uppercase no-underline cursor-pointer shadow-[0_24px_80px_rgb(0_0_0_/_22%)] opacity-75".into(),
                "border border-2 border-t border-b border-[rgb(247_244_237_/_12%)] bg-[linear-gradient(120deg,rgb(27_37_34_/_92%),#111414)]".into(),
            ],
        );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert!(result.css.contains(".overflow-hidden{overflow:hidden;}"));
    assert!(result.css.contains(".z-\\[1\\]{z-index:1;}"));
    assert!(result.css.contains(".leading-tight{line-height:1.1;}"));
    assert!(result
        .css
        .contains(".tracking-widest{letter-spacing:0.1em;}"));
    assert!(result.css.contains(".uppercase{text-transform:uppercase;}"));
    assert!(result.css.contains(".opacity-75{opacity:0.75;}"));
    assert!(result
        .css
        .contains(".border{border-width:1px;border-style:solid;}"));
    assert!(result
        .css
        .contains(".border-2{border-width:2px;border-style:solid;}"));
}

#[test]
fn compiles_common_tailwind_utilities() {
    let result = compile(
        &config(),
        &[
            "inline inline-flex inline-grid contents flow-root static visible invisible".into(),
            "outline outline-none outline-dashed outline-2 outline-accent outline-offset-2".into(),
            "rounded rounded-none rounded-full".into(),
            "underline line-through italic not-italic lowercase capitalize normal-case".into(),
            "truncate whitespace-nowrap whitespace-pre break-words break-all".into(),
            "text-ellipsis text-balance text-wrap text-nowrap text-justify".into(),
            "transition transition-none transition-colors transition-opacity ease-in ease-out ease-in-out duration-300 delay-150".into(),
            "object-cover object-contain object-center object-top aspect-square aspect-video".into(),
            "select-none select-all pointer-events-none pointer-events-auto".into(),
            "sr-only not-sr-only box-border box-content resize resize-none appearance-none".into(),
            "overflow-scroll overflow-visible overflow-x-hidden overflow-y-scroll".into(),
            "cursor-default cursor-not-allowed cursor-grab".into(),
            "shadow-none decoration-accent".into(),
        ],
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert!(result.css.contains(".inline{display:inline;}"));
    assert!(result.css.contains(".inline-flex{display:inline-flex;}"));
    assert!(result.css.contains(".inline-grid{display:inline-grid;}"));
    assert!(result.css.contains(".contents{display:contents;}"));
    assert!(result.css.contains(".static{position:static;}"));
    assert!(result.css.contains(".visible{visibility:visible;}"));
    assert!(result.css.contains(".invisible{visibility:hidden;}"));
    assert!(result.css.contains(".outline{outline-width:1px;outline-style:solid;}"));
    assert!(result.css.contains(".outline-none{outline:none;}"));
    assert!(result.css.contains(".outline-2{outline-width:2px;outline-style:solid;}"));
    assert!(result.css.contains(".outline-accent{outline-color:var(--color-accent);}"));
    assert!(result.css.contains(".outline-offset-2{outline-offset:2px;}"));
    assert!(result.css.contains(".rounded{border-radius:0.25rem;}"));
    assert!(result.css.contains(".rounded-none{border-radius:0;}"));
    assert!(result.css.contains(".rounded-full{border-radius:9999px;}"));
    assert!(result.css.contains(".underline{text-decoration-line:underline;}"));
    assert!(result.css.contains(".line-through{text-decoration-line:line-through;}"));
    assert!(result.css.contains(".italic{font-style:italic;}"));
    assert!(result.css.contains(".not-italic{font-style:normal;}"));
    assert!(result.css.contains(".lowercase{text-transform:lowercase;}"));
    assert!(result.css.contains(".capitalize{text-transform:capitalize;}"));
    assert!(result.css.contains(".truncate{overflow:hidden;text-overflow:ellipsis;white-space:nowrap;}"));
    assert!(result.css.contains(".whitespace-nowrap{white-space:nowrap;}"));
    assert!(result.css.contains(".break-all{word-break:break-all;}"));
    assert!(result.css.contains(".text-ellipsis{text-overflow:ellipsis;}"));
    assert!(result.css.contains(".text-balance{text-wrap:balance;}"));
    assert!(result.css.contains(".text-justify{text-align:justify;}"));
    assert!(result.css.contains("transition-property:color,background-color,"));
    assert!(result.css.contains(".transition-none{transition-property:none;}"));
    assert!(result.css.contains(".duration-300{transition-duration:300ms;}"));
    assert!(result.css.contains(".delay-150{transition-delay:150ms;}"));
    assert!(result.css.contains(".ease-in-out{transition-timing-function:cubic-bezier(0.4,0,0.2,1);}"));
    assert!(result.css.contains(".object-cover{object-fit:cover;}"));
    assert!(result.css.contains(".object-center{object-position:center;}"));
    assert!(result.css.contains(".aspect-square{aspect-ratio:1/1;}"));
    assert!(result.css.contains(".aspect-video{aspect-ratio:16/9;}"));
    assert!(result.css.contains(".select-none{user-select:none;}"));
    assert!(result.css.contains(".pointer-events-none{pointer-events:none;}"));
    assert!(result.css.contains(".sr-only{position:absolute;"));
    assert!(result.css.contains(".box-border{box-sizing:border-box;}"));
    assert!(result.css.contains(".resize-none{resize:none;}"));
    assert!(result.css.contains(".appearance-none{appearance:none;}"));
    assert!(result.css.contains(".overflow-scroll{overflow:scroll;}"));
    assert!(result.css.contains(".overflow-x-hidden{overflow-x:hidden;}"));
    assert!(result.css.contains(".cursor-not-allowed{cursor:not-allowed;}"));
    assert!(result.css.contains(".shadow-none{box-shadow:none;}"));
    assert!(result.css.contains(".decoration-accent{text-decoration-color:var(--color-accent);}"));
}

#[test]
fn reports_unsupported_utilities_without_panicking() {
    let result = compile(&config(), &["unknown".into()]);

    assert_eq!(result.errors.len(), 1);
    assert_eq!(
        result.errors[0].message,
        "unsupported utility `unknown`".to_owned()
    );
}

#[test]
fn parser_rejects_malformed_groups() {
    let errors = parse_classlist("hover:(bg-accent").unwrap_err();

    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0].message, "unclosed `(` in utility");
}
