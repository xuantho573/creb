// src/app.rs

use crate::epub::handler::EpubHandler;
use crate::epub::content::RenderableChapter;
use crate::epub::processor::process_chapter_html;

pub struct AppState {
    pub epub_handler: EpubHandler,
    pub current_chapter_index: usize,
    pub renderable_chapter: RenderableChapter,
    pub should_quit: bool,
    pub scroll_position: usize,
}

impl AppState {
    pub fn new(mut epub_handler: EpubHandler, initial_chapter: usize) -> Result<Self, String> {
        let raw_html = epub_handler.get_chapter_content_raw(initial_chapter)?;
        let renderable_chapter = process_chapter_html(&raw_html);
        
        Ok(AppState {
            epub_handler,
            current_chapter_index: initial_chapter,
            renderable_chapter,
            should_quit: false,
            scroll_position: 0,
        })
    }

    pub fn next_chapter(&mut self) -> Result<(), String> {
        if self.current_chapter_index + 1 < self.epub_handler.get_chapter_count() {
            self.current_chapter_index += 1;
            self.load_current_chapter()?;
            self.scroll_position = 0; // Reset scroll when changing chapters
        }
        Ok(())
    }

    pub fn previous_chapter(&mut self) -> Result<(), String> {
        if self.current_chapter_index > 0 {
            self.current_chapter_index -= 1;
            self.load_current_chapter()?;
            self.scroll_position = 0; // Reset scroll when changing chapters
        }
        Ok(())
    }

    fn load_current_chapter(&mut self) -> Result<(), String> {
        let raw_html = self.epub_handler.get_chapter_content_raw(self.current_chapter_index)?;
        self.renderable_chapter = process_chapter_html(&raw_html);
        Ok(())
    }

    pub fn scroll_down(&mut self) {
        // We'll implement scrolling in the renderer
        self.scroll_position = self.scroll_position.saturating_add(1);
    }

    pub fn scroll_up(&mut self) {
        self.scroll_position = self.scroll_position.saturating_sub(1);
    }

    pub fn page_down(&mut self, page_size: usize) {
        self.scroll_position = self.scroll_position.saturating_add(page_size);
    }

    pub fn page_up(&mut self, page_size: usize) {
        self.scroll_position = self.scroll_position.saturating_sub(page_size);
    }

    pub fn get_chapter_title(&self) -> String {
        // For now, we'll just return a generic title
        // In a more complete implementation, we would extract the actual chapter title
        format!("Chapter {}", self.current_chapter_index + 1)
    }

    pub fn get_chapter_progress(&self) -> f64 {
        if self.epub_handler.get_chapter_count() <= 1 {
            1.0
        } else {
            self.current_chapter_index as f64 / (self.epub_handler.get_chapter_count() - 1) as f64
        }
    }
}
