use serde::Deserialize;

#[derive(Deserialize)]
#[serde(tag = "type")]
enum Thing {
    Numby { _field: u8 },
    Stringy { _field: String },
    Booly { _field: bool },
    Enumy,
}

#[cfg(test)]
mod tests {
    use super::Thing;

    #[test]
    fn thing_numby() {
        let _thing: Thing =
            serde_json::from_str(r#"{ "type": "Numby", "_field": 1 }"#).expect("couldn't parse");
    }


    #[test]
    fn thing_stringy() {
        let _thing: Thing =
            serde_json::from_str(r#"{ "type": "Stringy", "_field": "val" }"#).expect("couldn't parse");
    }


    #[test]
    fn thing_booly() {
        let _thing: Thing =
            serde_json::from_str(r#"{ "type": "Booly", "_field": true }"#).expect("couldn't parse");
    }

    #[test]
    fn thing_enumy() {
        let _thing: Thing = serde_json::from_str(r#"{ "type": "Enumy" }"#).expect("couldn't parse");
    }
}
