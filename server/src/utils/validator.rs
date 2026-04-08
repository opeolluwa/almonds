pub fn format_validation_errors(e: validator::ValidationErrors) -> Vec<String> {
    e.field_errors()
        .iter()
        .flat_map(|(field, errors)| {
            errors.iter().map(move |err| {
                format!(
                    "{}: {}",
                    field,
                    err.message
                        .clone()
                        .unwrap_or_else(|| "Invalid value".into())
                )
            })
        })
        .collect()
}
