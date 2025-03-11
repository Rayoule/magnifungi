#[cfg(test)]

use magnifungi_macros::*;
use magnifungi_shared_types::entry_types::entry_enum::JsonEnum;


#[test]

fn check_enum_serde() {

    let json_input = r#"
        {
                "name":"Edible",
                "variants":["Yes","No","Maybe"]
        }"#;
    
    let json_enum = JsonEnum {
        name: "Edible".to_string(),
        variants: vec!["Yes".to_string(), "No".to_string(), "Maybe".to_string()],
    };

    let json_enum_from_input: JsonEnum = serde_json::from_str(json_input).unwrap();

    println!("json input: {:#?}", &json_enum);
    println!("json struct result: {:#?}", &json_enum_from_input);

    assert_eq!(json_enum, json_enum_from_input);
}

#[test]
fn check_macro() {
    use magnifungi_shared_types::entry_types::entry_enum::IntoEntryEnumerator;
    //use strum::IntoEnumIterator;
    use strum_macros::EnumIter;

    generate_enum_from_path!();

    println!("json struct result: {:#?}", Edible::Yes);

    assert_eq!(Edible::Yes, Edible::Yes);
}