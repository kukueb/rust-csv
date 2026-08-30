use syn::{Type, TypeReference};

#[derive(Debug)]
pub enum FieldType {
    StringLiteral,
    DateTime,
    Other(Option<String>),
}
pub fn classify(ty: &Type) -> FieldType {
    match ty {
        Type::Path(type_path) => {
            if let Some(segment) = type_path.path.segments.last() {
                let ident: String = segment.ident.to_string();
                return match ident.as_str() {
                    "NaiveDateTime" => FieldType::DateTime,
                    val => {
                        // println!("Strange type found in struct: {}", val);
                        return FieldType::Other(Some(String::from(val)));
                    }
                };
            }
        }
        Type::Reference(TypeReference { elem, .. }) => {
            if let Type::Path(p) = elem.as_ref() {
                if p.path.is_ident("str") {
                    return FieldType::StringLiteral;
                }
            }
        }
        _ => (),
    }

    return FieldType::Other(None);
}
