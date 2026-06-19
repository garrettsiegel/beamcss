use crate::{
    colors::color_declaration,
    emit::canonical_class_name,
    properties::{raw_declaration, size_property, spacing_property},
    typography::{
        font_declaration, letter_spacing_declaration, line_height_declaration, opacity_declaration,
        scale_declaration, size_declaration, text_declaration,
    },
    utility_modules::require_module,
    values::{css_space_value, is_css_number, raw_value, token_or_raw_declaration},
    variants::variant_effects,
    AtomRule, BeamConfig, ResolvedAtom,
};

pub(crate) fn compile_atom(config: &BeamConfig, atom: &ResolvedAtom) -> Result<AtomRule, String> {
    let declaration = declaration_for_base(config, &atom.base)?;
    let class_name = if atom.selector_class.is_empty() {
        canonical_class_name(&atom.variants, &atom.base)
    } else {
        atom.selector_class.clone()
    };
    let (wrappers, pseudos, selector_transform) = variant_effects(config, &atom.variants)?;

    Ok(AtomRule {
        layer: atom.layer,
        class_name,
        declaration,
        wrappers,
        pseudos,
        selector_transform,
    })
}

pub(crate) fn declaration_for_base(config: &BeamConfig, base: &str) -> Result<String, String> {
    if let Some(value) = base.strip_prefix("inset-x-") {
        require_module(config, "layout")?;
        let value = raw_value(value)?;
        return Ok(format!("left:{value};right:{value}"));
    }
    if let Some(value) = base.strip_prefix("inset-y-") {
        require_module(config, "layout")?;
        let value = raw_value(value)?;
        return Ok(format!("top:{value};bottom:{value}"));
    }
    if let Some(value) = base.strip_prefix("inset-") {
        require_module(config, "layout")?;
        return raw_declaration(value, "inset");
    }
    if let Some(value) = base.strip_prefix("top-") {
        require_module(config, "layout")?;
        return raw_declaration(value, "top");
    }
    if let Some(value) = base.strip_prefix("right-") {
        require_module(config, "layout")?;
        return raw_declaration(value, "right");
    }
    if let Some(value) = base.strip_prefix("bottom-") {
        require_module(config, "layout")?;
        return raw_declaration(value, "bottom");
    }
    if let Some(value) = base.strip_prefix("left-") {
        require_module(config, "layout")?;
        return raw_declaration(value, "left");
    }
    if let Some(value) = base.strip_prefix("z-") {
        require_module(config, "layout")?;
        return raw_declaration(value, "z-index");
    }
    if let Some(value) = base.strip_prefix("gap-x-") {
        require_module(config, "spacing")?;
        return space_declaration(config, value, "column-gap");
    }
    if let Some(value) = base.strip_prefix("gap-y-") {
        require_module(config, "spacing")?;
        return space_declaration(config, value, "row-gap");
    }
    if let Some(value) = base.strip_prefix("gap-") {
        require_module(config, "spacing")?;
        return space_declaration(config, value, "gap");
    }

    if let Some(value) = base.strip_prefix("bg-") {
        require_module(config, "colors")?;
        return color_declaration(value, &config.tokens.color, "background");
    }
    if let Some(value) = base.strip_prefix("rounded-") {
        require_module(config, "layout")?;
        return token_or_raw_declaration(value, &config.tokens.radius, "border-radius", "radius");
    }
    if let Some(value) = base.strip_prefix("text-") {
        require_module(config, text_module(value, config))?;
        return text_declaration(value, config);
    }
    if let Some(value) = base.strip_prefix("font-") {
        require_module(config, "typography")?;
        return font_declaration(config, value);
    }
    if let Some(value) = base.strip_prefix("leading-") {
        require_module(config, "typography")?;
        return line_height_declaration(value);
    }
    if let Some(value) = base.strip_prefix("tracking-") {
        require_module(config, "typography")?;
        return letter_spacing_declaration(value);
    }
    if let Some(value) = base.strip_prefix("opacity-") {
        require_module(config, "effects")?;
        return opacity_declaration(value);
    }
    if let Some(value) = base.strip_prefix("shadow-") {
        require_module(config, "effects")?;
        return raw_declaration(value, "box-shadow");
    }
    if let Some((property, value)) = spacing_property(base, "p") {
        require_module(config, "spacing")?;
        return space_declaration(config, value, property);
    }
    if let Some((property, value)) = spacing_property(base, "m") {
        require_module(config, "spacing")?;
        return space_declaration(config, value, property);
    }
    if let Some((property, value)) = size_property(base) {
        require_module(config, "layout")?;
        return size_declaration(value, property);
    }
    if let Some(value) = base.strip_prefix("scale-") {
        require_module(config, "effects")?;
        return scale_declaration(value);
    }
    if let Some(value) = base.strip_prefix("cols-") {
        require_module(config, "layout")?;
        return grid_track_declaration(value, "grid-template-columns");
    }
    if let Some(value) = base.strip_prefix("rows-") {
        require_module(config, "layout")?;
        return grid_track_declaration(value, "grid-template-rows");
    }

    match base {
        "flex" => layout_declaration(config, "display:flex"),
        "grid" => layout_declaration(config, "display:grid"),
        "inline-block" => layout_declaration(config, "display:inline-block"),
        "direction-column" => layout_declaration(config, "flex-direction:column"),
        "direction-row" => layout_declaration(config, "flex-direction:row"),
        "place-center" => layout_declaration(config, "place-items:center"),
        "wrap" => layout_declaration(config, "flex-wrap:wrap"),
        "nowrap" => layout_declaration(config, "flex-wrap:nowrap"),
        "align-start" => layout_declaration(config, "align-items:flex-start"),
        "align-end" => layout_declaration(config, "align-items:flex-end"),
        "align-stretch" => layout_declaration(config, "align-items:stretch"),
        "align-baseline" => layout_declaration(config, "align-items:baseline"),
        "align-center" => layout_declaration(config, "align-items:center"),
        "justify-start" => layout_declaration(config, "justify-content:flex-start"),
        "justify-center" => layout_declaration(config, "justify-content:center"),
        "justify-end" => layout_declaration(config, "justify-content:flex-end"),
        "justify-between" => layout_declaration(config, "justify-content:space-between"),
        "justify-around" => layout_declaration(config, "justify-content:space-around"),
        "justify-evenly" => layout_declaration(config, "justify-content:space-evenly"),
        "block" => layout_declaration(config, "display:block"),
        "hidden" => layout_declaration(config, "display:none"),
        "absolute" => layout_declaration(config, "position:absolute"),
        "relative" => layout_declaration(config, "position:relative"),
        "fixed" => layout_declaration(config, "position:fixed"),
        "sticky" => layout_declaration(config, "position:sticky"),
        "overflow-hidden" => layout_declaration(config, "overflow:hidden"),
        "overflow-auto" => layout_declaration(config, "overflow:auto"),
        "overflow-x-auto" => layout_declaration(config, "overflow-x:auto"),
        "overflow-y-auto" => layout_declaration(config, "overflow-y:auto"),
        "list-none" => typography_static_declaration(config, "list-style:none"),
        "uppercase" => typography_static_declaration(config, "text-transform:uppercase"),
        "text-left" => typography_static_declaration(config, "text-align:left"),
        "text-center" => typography_static_declaration(config, "text-align:center"),
        "text-right" => typography_static_declaration(config, "text-align:right"),
        "no-underline" => typography_static_declaration(config, "text-decoration:none"),
        "cursor-pointer" => layout_declaration(config, "cursor:pointer"),
        "border" => layout_declaration(config, "border-width:1px;border-style:solid"),
        "border-0" => layout_declaration(config, "border-width:0"),
        "border-solid" => layout_declaration(config, "border-style:solid"),
        "border-dashed" => layout_declaration(config, "border-style:dashed"),
        "border-dotted" => layout_declaration(config, "border-style:dotted"),
        "border-double" => layout_declaration(config, "border-style:double"),
        "border-none" => layout_declaration(config, "border-style:none"),
        "border-t" => layout_declaration(config, "border-top-width:1px;border-top-style:solid"),
        "border-b" => {
            layout_declaration(config, "border-bottom-width:1px;border-bottom-style:solid")
        }
        "border-l" => layout_declaration(config, "border-left-width:1px;border-left-style:solid"),
        "border-r" => layout_declaration(config, "border-right-width:1px;border-right-style:solid"),
        _ => {
            if let Some(value) = base.strip_prefix("border-") {
                if is_css_number(value) {
                    require_module(config, "layout")?;
                    return Ok(format!("border-width:{value}px;border-style:solid"));
                }
                require_module(config, "colors")?;
                return color_declaration(value, &config.tokens.color, "border-color");
            }
            Err(format!("unsupported utility `{base}`"))
        }
    }
}

fn text_module(value: &str, config: &BeamConfig) -> &'static str {
    if matches!(value, "left" | "center" | "right")
        || config.tokens.text.contains_key(value)
        || is_css_number(value)
    {
        "typography"
    } else {
        "colors"
    }
}

fn layout_declaration(config: &BeamConfig, declaration: &str) -> Result<String, String> {
    require_module(config, "layout")?;
    Ok(declaration.to_owned())
}

fn typography_static_declaration(config: &BeamConfig, declaration: &str) -> Result<String, String> {
    require_module(config, "typography")?;
    Ok(declaration.to_owned())
}

fn space_declaration(config: &BeamConfig, value: &str, property: &str) -> Result<String, String> {
    let value = css_space_value(value, &config.tokens.spacing)?;
    Ok(format!("{property}:{value}"))
}

fn grid_track_declaration(value: &str, property: &str) -> Result<String, String> {
    let value = if let Ok(count) = value.parse::<usize>() {
        format!("repeat({count},1fr)")
    } else {
        raw_value(value)?
    };
    Ok(format!("{property}:{value}"))
}
