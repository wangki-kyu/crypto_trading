#[macro_export]
macro_rules! query {
    ($($field:expr), *) => {{
        let mut query = String::new();
        $(
            if !query.is_empty() {
                query.push('&');
            }
            let mut name = stringify!($field).to_string();
            if name.contains("r#") {
                name = name.replace("r#", "");
            }
            query.push_str(&format!("{}={}", name, $field));
        )*
        query
    }
    };
}