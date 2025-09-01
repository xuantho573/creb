// src/epub/content.rs

#[derive(Debug)]
pub enum RenderableBlock {
  Paragraph(String),
  Heading(usize, String),   // usize for heading level (h1, h2, etc.)
  Image(String),            // Path or URL to the image
  ImagePlaceholder(String), // For images that couldn't be loaded
}

#[derive(Debug)]
pub struct RenderableChapter {
  pub blocks: Vec<RenderableBlock>,
}
