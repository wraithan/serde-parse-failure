use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type")]
enum Thing {
    Structy { field: u8 },
    Enumy,
}

#[cfg(test)]
mod tests {
    use super::Thing;

    #[test]
    fn thing_structy() {
        let _thing: Thing =
            serde_json::from_str(r#"{ "type": "Structy", "field": 1 }"#).expect("couldn't parse");
    }

    #[test]
    fn thing_enumy() {
        let _thing: Thing = serde_json::from_str(r#"{ "type": "Enumy" }"#).expect("couldn't parse");
    }
}
