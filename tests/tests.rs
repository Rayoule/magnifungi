#[cfg(test)]


#[test]
fn enum_macro_test() {
    use strum_macros::EnumIter;
    use magnifungi_shared_types::entry_types::entry_enum::IntoEntryEnum;
    use magnifungi_macros::generate_enums_from_path;
    generate_enums_from_path!();
}