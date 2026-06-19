use crate::{parser, BeamConfig, ClassToken};

const MAX_EXPANSION_DEPTH: usize = 16;

pub(crate) fn expand_token(config: &BeamConfig, token: &ClassToken) -> Result<ClassToken, String> {
    expand_token_inner(config, token, &mut Vec::new())
}

fn expand_token_inner(
    config: &BeamConfig,
    token: &ClassToken,
    stack: &mut Vec<String>,
) -> Result<ClassToken, String> {
    if stack.len() > MAX_EXPANSION_DEPTH {
        return Err("class expansion exceeded maximum depth".to_owned());
    }

    match token {
        ClassToken::Utility { variants, base } => expand_utility(config, variants, base, stack),
        ClassToken::Group { variants, children } => Ok(ClassToken::Group {
            variants: variants.clone(),
            children: expand_children(config, children, stack)?,
        }),
    }
}

fn expand_utility(
    config: &BeamConfig,
    variants: &[String],
    base: &str,
    stack: &mut Vec<String>,
) -> Result<ClassToken, String> {
    if let Some((outer_variants, recipe, variant)) = recipe_variant(config, variants, base) {
        return expand_recipe_variant(config, outer_variants, recipe, variant, stack);
    }

    if let Some(recipe) = config.recipes.get(base) {
        return expand_recipe_base(config, variants, base, &recipe.base, stack);
    }

    if let Some(shortcut) = config.shortcuts.get(base) {
        return expand_named_class(config, variants, "shortcut", base, shortcut, stack);
    }

    Ok(ClassToken::Utility {
        variants: variants.to_vec(),
        base: base.to_owned(),
    })
}

fn expand_recipe_variant(
    config: &BeamConfig,
    outer_variants: &[String],
    recipe_name: &str,
    variant_name: &str,
    stack: &mut Vec<String>,
) -> Result<ClassToken, String> {
    let recipe = &config.recipes[recipe_name];
    let variant = recipe
        .variants
        .get(variant_name)
        .ok_or_else(|| format!("recipe `{recipe_name}` has no variant `{variant_name}`"))?;
    let classlist = if recipe.base.is_empty() {
        variant.clone()
    } else {
        format!("{} {}", recipe.base, variant)
    };

    expand_named_class(
        config,
        outer_variants,
        "recipe",
        &format!("{recipe_name}:{variant_name}"),
        &classlist,
        stack,
    )
}

fn expand_recipe_base(
    config: &BeamConfig,
    variants: &[String],
    name: &str,
    classlist: &str,
    stack: &mut Vec<String>,
) -> Result<ClassToken, String> {
    expand_named_class(config, variants, "recipe", name, classlist, stack)
}

fn expand_named_class(
    config: &BeamConfig,
    variants: &[String],
    kind: &str,
    name: &str,
    classlist: &str,
    stack: &mut Vec<String>,
) -> Result<ClassToken, String> {
    let stack_key = format!("{kind}:{name}");
    if stack.iter().any(|item| item == &stack_key) {
        return Err(format!("{kind} `{name}` expands recursively"));
    }

    stack.push(stack_key);
    let parsed = parser::parse_classlist(classlist).map_err(|errors| {
        errors
            .into_iter()
            .map(|error| format!("{}: {}", error.class_name, error.message))
            .collect::<Vec<_>>()
            .join("; ")
    })?;
    let children = expand_children(config, &parsed, stack)?;
    stack.pop();

    Ok(ClassToken::Group {
        variants: variants.to_vec(),
        children,
    })
}

fn recipe_variant<'a>(
    config: &'a BeamConfig,
    variants: &'a [String],
    base: &'a str,
) -> Option<(&'a [String], &'a str, &'a str)> {
    let (recipe_name, outer_variants) = variants.split_last()?;
    config
        .recipes
        .contains_key(recipe_name)
        .then_some((outer_variants, recipe_name.as_str(), base))
}

fn expand_children(
    config: &BeamConfig,
    children: &[ClassToken],
    stack: &mut Vec<String>,
) -> Result<Vec<ClassToken>, String> {
    children
        .iter()
        .map(|child| expand_token_inner(config, child, stack))
        .collect()
}
