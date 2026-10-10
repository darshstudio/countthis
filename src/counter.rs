use std::fs;
use std::io::Error;

#[derive(Debug)]
pub struct Counter {
    options: CountingMode,
    filepath: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum CountingMode {
    Words,
    Lines,
}

impl Counter {
    pub fn set_counter(options: &CountingMode, filepath: &str) -> Self {
        Self {
            options: options.clone(),
            filepath: filepath.to_string(),
        }
    }

    pub fn get_count(&self) -> Result<usize, Error> {
        let content = fs::read_to_string(&self.filepath)?;
        if self.options == CountingMode::Words {
            Ok(content.split_whitespace().count())
        } else {
            Ok(content.lines().count())
        }
    }
}