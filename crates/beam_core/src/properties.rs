use crate::values::raw_value;

pub(crate) fn raw_declaration(value: &str, property: &str) -> Result<String, String> {
    let value = raw_value(value)?;
    Ok(format!("{property}:{value}"))
}

pub(crate) fn spacing_property<'a>(
    class_name: &'a str,
    family: &str,
) -> Option<(&'static str, &'a str)> {
    let mappings = match family {
        "p" => [
            ("px-", "padding-inline"),
            ("py-", "padding-block"),
            ("pt-", "padding-top"),
            ("pr-", "padding-right"),
            ("pb-", "padding-bottom"),
            ("pl-", "padding-left"),
            ("p-", "padding"),
        ],
        "m" => [
            ("mx-", "margin-inline"),
            ("my-", "margin-block"),
            ("mt-", "margin-top"),
            ("mr-", "margin-right"),
            ("mb-", "margin-bottom"),
            ("ml-", "margin-left"),
            ("m-", "margin"),
        ],
        _ => return None,
    };

    mappings.iter().find_map(|(prefix, property)| {
        class_name
            .strip_prefix(prefix)
            .map(|value| (*property, value))
    })
}

pub(crate) fn size_property(class_name: &str) -> Option<(&'static str, &str)> {
    [
        ("min-w-", "min-width"),
        ("min-h-", "min-height"),
        ("max-w-", "max-width"),
        ("max-h-", "max-height"),
        ("w-", "width"),
        ("h-", "height"),
    ]
    .iter()
    .find_map(|(prefix, property)| {
        class_name
            .strip_prefix(prefix)
            .map(|value| (*property, value))
    })
}
