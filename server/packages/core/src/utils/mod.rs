pub mod check;
pub mod headers {
    use proto::core::Header;

    pub fn headers_from(filename: &str) -> Vec<Header> {
        let gress = mime_guess::from_path(filename);
        match gress.first() {
            Some(name) => {
                vec![Header {
                    key: "Content-Type".to_string(),
                    value: name.to_string(),
                }]
            }
            None => vec![],
        }
    }
}
