use crate::pdf::PDF;

struct Library {
    pdfs: Vec<PDF>,
}

impl Library {
    fn new() -> Self {
        let pdfs = Vec::new();
        Self { pdfs }
    }
}
