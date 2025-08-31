// src/epub/handler.rs

use std::path::PathBuf;
use std::fs::File;
use std::io::BufReader;
use epub::doc::EpubDoc;

pub struct EpubHandler {
    doc: EpubDoc<BufReader<File>>,
}

impl EpubHandler {
    pub fn new(path: PathBuf) -> Result<Self, String> {
        let doc = EpubDoc::new(path).map_err(|e| format!("Failed to open EPUB: {}", e))?;
        Ok(EpubHandler { doc })
    }

    pub fn get_chapter_count(&self) -> usize {
        self.doc.get_num_pages()
    }

    pub fn get_chapter_content_raw(&mut self, chapter_index: usize) -> Result<String, String> {
        if chapter_index >= self.get_chapter_count() {
            return Err(format!("Chapter index {} out of bounds", chapter_index));
        }

        if !self.doc.set_current_page(chapter_index) {
            return Err(format!("Failed to set current chapter to {}", chapter_index));
        }
        
        // Get the current chapter content
        match self.doc.get_current() {
            Some(current) => {
                // The current object contains the raw bytes in .0
                // Let's try to convert it to a string
                String::from_utf8(current.0)
                    .map_err(|_| "Failed to decode chapter content as UTF-8".to_string())
            }
            None => Err("Failed to get chapter content".to_string())
        }
    }
}
