pub fn name_from_administation_code(code: &str) -> Option<&str> {
    match code {
        "80" => Some("DB Fernverkehr AG"),
        "82" => Some("CFL"),
        "87" => Some("SNCF"),
        "88" => Some("SNCB"),
        _ => None,
    }
}
