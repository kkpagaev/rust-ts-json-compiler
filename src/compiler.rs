use crate::syntax_tree::ZodExpression;

pub fn to_json(zod: &ZodExpression) -> String {
    match zod {
        ZodExpression::Object(obj) => {
            let mut json = String::new();
            json.push('{');
            json.push_str(
                &obj.iter()
                    .map(|(key, value)| return format!("\"{}\": {}", key, to_json(value)))
                    .collect::<Vec<String>>()
                    .join(", "),
            );

            json.push('}');
            return json;
        }
        ZodExpression::Number => return "1".to_string(),
        ZodExpression::String => return "\"string\"".to_string(),
        ZodExpression::UUID   => return "\"aa5ac446-7e1d-11ee-b962-0242ac120002\"".to_string(),
        ZodExpression::Boolean => return "true".to_string(),
        ZodExpression::Array(array) => {
            let mut json = String::new();
            json.push('[');
            json.push_str(&to_json(array));
            json.push(']');
            json
        },
        ZodExpression::Literal(l) => return format!("\"{}\"", l),
        ZodExpression::Email => return "\"admin@admin.com\"".to_string(),
        ZodExpression::Any => return "{}".to_string(),
        ZodExpression::Enum(e) => return format!("\"{}\"", e.first().unwrap()),
        ZodExpression::Union(u) => return to_json(u.first().unwrap()),
    }
}
