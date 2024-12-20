pub struct WriteSomeing {
    text: String,
}

impl WriteSomeing {
    pub fn new(text: String) -> Self {
        Self { text }
    }

    pub fn show(&self) {
        println!("{}", self.text)
    }
}
