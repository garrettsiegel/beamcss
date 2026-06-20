use crate::{
    colors::color_declaration,
    emit::canonical_class_name,
    properties::{raw_declaration, size_property, spacing_property},
    typography::{
        font_declaration, letter_spacing_declaration, line_height_declaration, opacity_declaration,
        scale_declaration, size_declaration, text_declaration,
    },
    utility_modules::require_module,
    values::{
        arbitrary_value, css_space_value, dynamic_value, is_css_number, raw_value,
        token_or_raw_declaration,
    },
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
        if !config.tokens.radius.contains_key(value) {
            match value {
                "none" => return Ok("border-radius:0".to_owned()),
                "full" => return Ok("border-radius:9999px".to_owned()),
                _ => {}
            }
        }
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
        if value == "none" {
            return Ok("box-shadow:none".to_owned());
        }
        return raw_declaration(value, "box-shadow");
    }
    if let Some(value) = base.strip_prefix("decoration-") {
        require_module(config, "typography")?;
        return color_declaration(value, &config.tokens.color, "text-decoration-color");
    }
    if let Some(value) = base.strip_prefix("outline-offset-") {
        require_module(config, "layout")?;
        if is_css_number(value) {
            return Ok(format!("outline-offset:{value}px"));
        }
        return raw_declaration(value, "outline-offset");
    }
    if let Some(value) = base.strip_prefix("duration-") {
        require_module(config, "effects")?;
        return ms_declaration(value, "transition-duration");
    }
    if let Some(value) = base.strip_prefix("delay-") {
        require_module(config, "effects")?;
        return ms_declaration(value, "transition-delay");
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
        // --- display ---
        "flex" => layout_declaration(config, "display:flex"),
        "grid" => layout_declaration(config, "display:grid"),
        "block" => layout_declaration(config, "display:block"),
        "inline" => layout_declaration(config, "display:inline"),
        "inline-block" => layout_declaration(config, "display:inline-block"),
        "inline-flex" => layout_declaration(config, "display:inline-flex"),
        "inline-grid" => layout_declaration(config, "display:inline-grid"),
        "contents" => layout_declaration(config, "display:contents"),
        "flow-root" => layout_declaration(config, "display:flow-root"),
        "hidden" => layout_declaration(config, "display:none"),
        // --- position / visibility ---
        "absolute" => layout_declaration(config, "position:absolute"),
        "relative" => layout_declaration(config, "position:relative"),
        "fixed" => layout_declaration(config, "position:fixed"),
        "sticky" => layout_declaration(config, "position:sticky"),
        "static" => layout_declaration(config, "position:static"),
        "visible" => layout_declaration(config, "visibility:visible"),
        "invisible" => layout_declaration(config, "visibility:hidden"),
        // --- flex / grid ---
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
        // --- overflow ---
        "overflow-hidden" => layout_declaration(config, "overflow:hidden"),
        "overflow-auto" => layout_declaration(config, "overflow:auto"),
        "overflow-scroll" => layout_declaration(config, "overflow:scroll"),
        "overflow-visible" => layout_declaration(config, "overflow:visible"),
        "overflow-x-auto" => layout_declaration(config, "overflow-x:auto"),
        "overflow-x-hidden" => layout_declaration(config, "overflow-x:hidden"),
        "overflow-x-scroll" => layout_declaration(config, "overflow-x:scroll"),
        "overflow-y-auto" => layout_declaration(config, "overflow-y:auto"),
        "overflow-y-hidden" => layout_declaration(config, "overflow-y:hidden"),
        "overflow-y-scroll" => layout_declaration(config, "overflow-y:scroll"),
        // --- outline ---
        "outline" => layout_declaration(config, "outline-width:1px;outline-style:solid"),
        "outline-none" => layout_declaration(config, "outline:none"),
        "outline-dashed" => layout_declaration(config, "outline-style:dashed"),
        "outline-dotted" => layout_declaration(config, "outline-style:dotted"),
        "outline-double" => layout_declaration(config, "outline-style:double"),
        // --- border ---
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
        // --- rounded (no suffix) ---
        "rounded" => layout_declaration(config, "border-radius:0.25rem"),
        // --- object fit / position ---
        "object-contain" => layout_declaration(config, "object-fit:contain"),
        "object-cover" => layout_declaration(config, "object-fit:cover"),
        "object-fill" => layout_declaration(config, "object-fit:fill"),
        "object-none" => layout_declaration(config, "object-fit:none"),
        "object-scale-down" => layout_declaration(config, "object-fit:scale-down"),
        "object-center" => layout_declaration(config, "object-position:center"),
        "object-top" => layout_declaration(config, "object-position:top"),
        "object-bottom" => layout_declaration(config, "object-position:bottom"),
        "object-left" => layout_declaration(config, "object-position:left"),
        "object-right" => layout_declaration(config, "object-position:right"),
        "object-left-top" => layout_declaration(config, "object-position:left top"),
        "object-right-top" => layout_declaration(config, "object-position:right top"),
        "object-left-bottom" => layout_declaration(config, "object-position:left bottom"),
        "object-right-bottom" => layout_declaration(config, "object-position:right bottom"),
        // --- aspect ratio ---
        "aspect-auto" => layout_declaration(config, "aspect-ratio:auto"),
        "aspect-square" => layout_declaration(config, "aspect-ratio:1/1"),
        "aspect-video" => layout_declaration(config, "aspect-ratio:16/9"),
        // --- box sizing ---
        "box-border" => layout_declaration(config, "box-sizing:border-box"),
        "box-content" => layout_declaration(config, "box-sizing:content-box"),
        // --- resize ---
        "resize" => layout_declaration(config, "resize:both"),
        "resize-none" => layout_declaration(config, "resize:none"),
        "resize-x" => layout_declaration(config, "resize:horizontal"),
        "resize-y" => layout_declaration(config, "resize:vertical"),
        // --- user select ---
        "select-none" => layout_declaration(config, "user-select:none"),
        "select-text" => layout_declaration(config, "user-select:text"),
        "select-all" => layout_declaration(config, "user-select:all"),
        "select-auto" => layout_declaration(config, "user-select:auto"),
        // --- pointer events ---
        "pointer-events-none" => layout_declaration(config, "pointer-events:none"),
        "pointer-events-auto" => layout_declaration(config, "pointer-events:auto"),
        // --- cursor ---
        "cursor-pointer" => layout_declaration(config, "cursor:pointer"),
        "cursor-default" => layout_declaration(config, "cursor:default"),
        "cursor-text" => layout_declaration(config, "cursor:text"),
        "cursor-wait" => layout_declaration(config, "cursor:wait"),
        "cursor-crosshair" => layout_declaration(config, "cursor:crosshair"),
        "cursor-not-allowed" => layout_declaration(config, "cursor:not-allowed"),
        "cursor-grab" => layout_declaration(config, "cursor:grab"),
        "cursor-grabbing" => layout_declaration(config, "cursor:grabbing"),
        "cursor-move" => layout_declaration(config, "cursor:move"),
        "cursor-help" => layout_declaration(config, "cursor:help"),
        // --- appearance ---
        "appearance-none" => layout_declaration(config, "appearance:none"),
        "appearance-auto" => layout_declaration(config, "appearance:auto"),
        // --- screen reader ---
        "sr-only" => layout_declaration(
            config,
            "position:absolute;width:1px;height:1px;padding:0;margin:-1px;overflow:hidden;clip:rect(0,0,0,0);white-space:nowrap;border-width:0",
        ),
        "not-sr-only" => layout_declaration(
            config,
            "position:static;width:auto;height:auto;padding:0;margin:0;overflow:visible;clip:auto;white-space:normal",
        ),
        // --- typography ---
        "list-none" => typography_static_declaration(config, "list-style:none"),
        "uppercase" => typography_static_declaration(config, "text-transform:uppercase"),
        "lowercase" => typography_static_declaration(config, "text-transform:lowercase"),
        "capitalize" => typography_static_declaration(config, "text-transform:capitalize"),
        "normal-case" => typography_static_declaration(config, "text-transform:none"),
        "underline" => typography_static_declaration(config, "text-decoration-line:underline"),
        "line-through" => typography_static_declaration(config, "text-decoration-line:line-through"),
        "no-underline" => typography_static_declaration(config, "text-decoration-line:none"),
        "italic" => typography_static_declaration(config, "font-style:italic"),
        "not-italic" => typography_static_declaration(config, "font-style:normal"),
        "truncate" => typography_static_declaration(
            config,
            "overflow:hidden;text-overflow:ellipsis;white-space:nowrap",
        ),
        "whitespace-normal" => typography_static_declaration(config, "white-space:normal"),
        "whitespace-nowrap" => typography_static_declaration(config, "white-space:nowrap"),
        "whitespace-pre" => typography_static_declaration(config, "white-space:pre"),
        "whitespace-pre-line" => typography_static_declaration(config, "white-space:pre-line"),
        "whitespace-pre-wrap" => typography_static_declaration(config, "white-space:pre-wrap"),
        "whitespace-break-spaces" => {
            typography_static_declaration(config, "white-space:break-spaces")
        }
        "break-words" => typography_static_declaration(config, "overflow-wrap:break-word"),
        "break-all" => typography_static_declaration(config, "word-break:break-all"),
        "break-normal" => {
            typography_static_declaration(config, "overflow-wrap:normal;word-break:normal")
        }
        "break-keep" => typography_static_declaration(config, "word-break:keep-all"),
        // --- transitions ---
        "transition" => effects_declaration(
            config,
            "transition-property:color,background-color,border-color,text-decoration-color,fill,stroke,opacity,box-shadow,transform,filter,backdrop-filter;transition-timing-function:cubic-bezier(0.4,0,0.2,1);transition-duration:150ms",
        ),
        "transition-none" => effects_declaration(config, "transition-property:none"),
        "transition-all" => effects_declaration(
            config,
            "transition-property:all;transition-timing-function:cubic-bezier(0.4,0,0.2,1);transition-duration:150ms",
        ),
        "transition-colors" => effects_declaration(
            config,
            "transition-property:color,background-color,border-color,text-decoration-color,fill,stroke;transition-timing-function:cubic-bezier(0.4,0,0.2,1);transition-duration:150ms",
        ),
        "transition-opacity" => effects_declaration(
            config,
            "transition-property:opacity;transition-timing-function:cubic-bezier(0.4,0,0.2,1);transition-duration:150ms",
        ),
        "transition-shadow" => effects_declaration(
            config,
            "transition-property:box-shadow;transition-timing-function:cubic-bezier(0.4,0,0.2,1);transition-duration:150ms",
        ),
        "transition-transform" => effects_declaration(
            config,
            "transition-property:transform;transition-timing-function:cubic-bezier(0.4,0,0.2,1);transition-duration:150ms",
        ),
        "ease-linear" => effects_declaration(config, "transition-timing-function:linear"),
        "ease-in" => {
            effects_declaration(config, "transition-timing-function:cubic-bezier(0.4,0,1,1)")
        }
        "ease-out" => {
            effects_declaration(config, "transition-timing-function:cubic-bezier(0,0,0.2,1)")
        }
        "ease-in-out" => {
            effects_declaration(config, "transition-timing-function:cubic-bezier(0.4,0,0.2,1)")
        }
        _ => {
            if let Some(value) = base.strip_prefix("border-") {
                if is_css_number(value) {
                    require_module(config, "layout")?;
                    return Ok(format!("border-width:{value}px;border-style:solid"));
                }
                require_module(config, "colors")?;
                return color_declaration(value, &config.tokens.color, "border-color");
            }
            if let Some(value) = base.strip_prefix("outline-") {
                if is_css_number(value) {
                    require_module(config, "layout")?;
                    return Ok(format!("outline-width:{value}px;outline-style:solid"));
                }
                require_module(config, "layout")?;
                return color_declaration(value, &config.tokens.color, "outline-color");
            }
            if let Some(value) = base.strip_prefix("aspect-") {
                require_module(config, "layout")?;
                return raw_declaration(value, "aspect-ratio");
            }
            Err(format!("unsupported utility `{base}`"))
        }
    }
}

fn text_module(value: &str, config: &BeamConfig) -> &'static str {
    if matches!(
        value,
        "left" | "center" | "right" | "justify" | "ellipsis" | "clip" | "wrap" | "nowrap"
            | "balance" | "pretty"
    ) || config.tokens.text.contains_key(value)
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

fn effects_declaration(config: &BeamConfig, declaration: &str) -> Result<String, String> {
    require_module(config, "effects")?;
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

fn ms_declaration(value: &str, property: &str) -> Result<String, String> {
    if let Some(raw) = dynamic_value(value) {
        return Ok(format!("{property}:{raw}"));
    }
    if let Some(raw) = arbitrary_value(value) {
        return Ok(format!("{property}:{raw}"));
    }
    if let Ok(ms) = value.parse::<u32>() {
        return Ok(format!("{property}:{ms}ms"));
    }
    Err(format!("unsupported duration value `{value}`"))
}
