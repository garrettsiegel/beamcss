use super::*;

#[test]
fn color_algebra_alpha_shade_and_mix() {
    let result = compile(
            &config(),
            &["bg-surface bg-accent/50 text-accent+10 bg-surface-20 bg-accent~surface bg-on-accent bg-on-accent-10 bg-accent-10/50".into()],
        );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    // plain token
    assert!(result
        .css
        .contains(".bg-surface{background:var(--color-surface);}"));
    // alpha
    assert!(result.css.contains(
        ".bg-accent\\/50{background:color-mix(in oklab,var(--color-accent) 50%,transparent);}"
    ));
    // lighten
    assert!(result
        .css
        .contains(".text-accent\\+10{color:color-mix(in oklab,var(--color-accent),white 10%);}"));
    // darken
    assert!(result.css.contains(
        ".bg-surface-20{background:color-mix(in oklab,var(--color-surface),black 20%);}"
    ));
    // mix two tokens
    assert!(result.css.contains(
            ".bg-accent\\~surface{background:color-mix(in oklab,var(--color-accent),var(--color-surface));}"
        ));
    // hyphenated token stays intact (longest match wins)
    assert!(result
        .css
        .contains(".bg-on-accent{background:var(--color-on-accent);}"));
    // darken applied to a hyphenated token
    assert!(result.css.contains(
        ".bg-on-accent-10{background:color-mix(in oklab,var(--color-on-accent),black 10%);}"
    ));
    // shade composed with alpha
    assert!(result.css.contains(
            ".bg-accent-10\\/50{background:color-mix(in oklab,color-mix(in oklab,var(--color-accent),black 10%) 50%,transparent);}"
        ));
}

#[test]
fn color_algebra_arbitrary_and_dynamic_pass_through() {
    let result = compile(
        &config(),
        &["bg-[oklch(72%_0.14_240)] text-(--brand)".into()],
    );

    assert!(result.errors.is_empty(), "{:?}", result.errors);
    assert!(result.css.contains("background:oklch(72% 0.14 240);"));
    assert!(result.css.contains("color:var(--brand);"));
}

#[test]
fn color_algebra_reports_undefined_token() {
    let result = compile(&config(), &["bg-missing text-accent+bogus".into()]);
    assert_eq!(result.errors.len(), 2);
    assert_eq!(
        result.errors[0].message,
        "color token `missing` is not defined"
    );
    assert_eq!(
        result.errors[1].message,
        "color token `accent+bogus` is not defined"
    );
}
