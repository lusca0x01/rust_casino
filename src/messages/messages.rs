pub struct Send {
    pub hash: String,
    pub point: f64,
}

pub trait Parse: Sized {
    fn dump(&self) -> String;
}

impl Parse for Send {
    fn dump(&self) -> String {
        serde_json::json!({
            "hash": self.hash,
            "point": self.point
        }
        )
        .to_string()
    }
}
