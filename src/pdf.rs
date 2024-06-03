use std::io::ErrorKind;
use std::path::Path;

pub struct PDF {
    path: Box<Path>,
}

impl PDF {
    fn new(path: String) -> Result<Self, ErrorKind> {
        let path = Path::new(&path);
        if !path.exists() {
            return Err(ErrorKind::NotFound);
        }

        Ok(Self { path: path.into() })
    }
}
