// src/epub/content.rs

#[derive(Debug)]
pub enum RenderableBlock {
    Paragraph(String),
    Heading(usize, String), // usize for heading level (h1, h2, etc.)
    // Add more types as needed, e.g., ListItem(String), ImagePlaceholder(String)
}

#[derive(Debug)]
pub struct RenderableChapter {
    pub blocks: Vec<RenderableBlock>,
}
