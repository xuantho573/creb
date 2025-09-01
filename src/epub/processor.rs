use super::content::{RenderableChapter, RenderableBlock};
use xml::reader::{EventReader, XmlEvent};

pub fn process_chapter_html(html_content: &str) -> RenderableChapter {
    let mut blocks = Vec::new();
    let mut current_text = String::new();
    let mut heading_level = 0;
    
    // Preprocess the HTML to make it more parseable
    let processed_html = preprocess_html(html_content);
    
    let parser = EventReader::from_str(&processed_html);
    
    for event in parser {
        match event {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                match name.local_name.as_str() {
                    "h1" => {
                        heading_level = 1;
                        current_text.clear();
                    }
                    "h2" => {
                        heading_level = 2;
                        current_text.clear();
                    }
                    "h3" => {
                        heading_level = 3;
                        current_text.clear();
                    }
                    "h4" => {
                        heading_level = 4;
                        current_text.clear();
                    }
                    "h5" => {
                        heading_level = 5;
                        current_text.clear();
                    }
                    "h6" => {
                        heading_level = 6;
                        current_text.clear();
                    }
                    "p" => {
                        // Start of a paragraph
                        current_text.clear();
                    }
                    "img" => {
                        if let Some(src_attr) = attributes.iter().find(|attr| attr.name.local_name == "src") {
                            blocks.push(RenderableBlock::Image(src_attr.value.clone().replace(r"../", "")));
                        } else {
                            blocks.push(RenderableBlock::ImagePlaceholder("Image without source".to_string()));
                        }
                    }
                    "image" => {
                         if let Some(src_attr) = attributes.iter().find(|attr| attr.name.local_name == "href") {
                            blocks.push(RenderableBlock::Image(src_attr.value.clone().replace(r"../", "")));
                        } else {
                            blocks.push(RenderableBlock::ImagePlaceholder("Image without source".to_string()));
                        }
                    }
                    _ => {
                        // For other elements, we don't need special handling
                    }
                }
            }
            Ok(XmlEvent::EndElement { name }) => {
                match name.local_name.as_str() {
                    "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                        if !current_text.trim().is_empty() {
                            blocks.push(RenderableBlock::Heading(heading_level, current_text.trim().to_string()));
                        }
                        current_text.clear();
                        heading_level = 0;
                    }
                    "p" => {
                        if !current_text.trim().is_empty() {
                            blocks.push(RenderableBlock::Paragraph(current_text.trim().to_string()));
                        }
                        current_text.clear();
                    }
                    _ => {
                        // For other elements, we don't need special handling
                    }
                }
            }
            Ok(XmlEvent::Characters(text)) => {
                current_text.push_str(&text);
            }
            Err(e) => {
                // If we encounter an error, fall back to the simple approach
                eprintln!("XML parsing error: {:?}", e);
                return fallback_processing(html_content);
            }
            _ => {}
        }
    }
    
    // If we didn't find any blocks, use the fallback
    if blocks.is_empty() {
        return fallback_processing(html_content);
    }
    
    RenderableChapter { blocks }
}

fn preprocess_html(html_content: &str) -> String {
    let mut content = html_content.to_string();
    
    // Remove DOCTYPE declaration
    if let Some(pos) = content.find("<!DOCTYPE") {
        if let Some(end_pos) = content[pos..].find('>') {
            content.replace_range(pos..pos+end_pos+1, "");
        }
    }
    
    // Remove XML declaration
    if let Some(pos) = content.find("<?xml") {
        if let Some(end_pos) = content[pos..].find('>') {
            content.replace_range(pos..pos+end_pos+1, "");
        }
    }
    
    content
}

fn fallback_processing(html_content: &str) -> RenderableChapter {
    // Preprocess the HTML content
    let processed_content = preprocess_html(html_content);
    
    // Simple fallback that treats the entire content as a paragraph
    RenderableChapter {
        blocks: vec![RenderableBlock::Paragraph(processed_content.to_string())],
    }
}
