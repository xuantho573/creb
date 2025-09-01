use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::BufReader;
use epub::doc::EpubDoc;

pub struct EpubHandler {
    pub doc: EpubDoc<BufReader<File>>,
    pub base_path: PathBuf,
    current_chapter_path: Option<PathBuf>,
}

impl EpubHandler {
    pub fn new(path: PathBuf) -> Result<Self, String> {
        let doc = EpubDoc::new(path.clone()).map_err(|e| format!("Failed to open EPUB: {} - path: {:?}", e, path))?;
        Ok(EpubHandler { 
            doc,
            base_path: path,
            current_chapter_path: None,
        })
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
        
        // Store the current chapter path for resolving relative image paths
        if let Some(current) = self.doc.get_current() {
            self.current_chapter_path = Some(PathBuf::from(&current.1));
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

    /// Resolve a relative path based on the current chapter's location
    /// 
    /// # Arguments
    /// 
    /// * `relative_path` - The relative path to resolve (e.g., "../images/cover.png")
    /// 
    /// # Returns
    /// 
    /// * `Ok(String)` - The resolved path that can be used to look up the resource
    /// * `Err(String)` - If the path cannot be resolved
    pub fn resolve_relative_path(&self, relative_path: &str) -> Result<String, String> {
        if let Some(chapter_path) = &self.current_chapter_path {
            // Parse the relative path
            let rel_path = Path::new(relative_path);
            
            // Resolve the relative path against the chapter's directory
            let chapter_dir = chapter_path.parent().unwrap_or_else(|| Path::new(""));
            let resolved_path = chapter_dir.join(rel_path);
            
            // Normalize the path to remove .. components
            let normalized_path = resolved_path.components().collect::<PathBuf>();
            
            // Convert back to string
            normalized_path.to_str()
                .map(|s| s.to_string())
                .ok_or_else(|| "Failed to convert resolved path to string".to_string())
        } else {
            Err("No current chapter path set".to_string())
        }
    }

    /// Extract a resource from the EPUB and save it to a temporary file
    /// 
    /// # Arguments
    /// 
    /// * `resource_path` - The relative path to the resource as found in the HTML
    /// 
    /// # Returns
    /// 
    /// * `Ok(PathBuf)` - Path to the temporary file containing the resource
    /// * `Err(String)` - If the resource could not be extracted
    pub fn extract_resource(&mut self, resource_path: &str) -> Result<PathBuf, String> {
        // First, try to resolve the path if it's relative
        let resolved_path = self.resolve_relative_path(resource_path).unwrap_or_else(|_| resource_path.to_string());
        
        // Collect resource keys to avoid borrowing issues
        let resource_keys: Vec<String> = self.doc.resources.keys().cloned().collect();
        
        // Look up the resource in the EPUB's resources map
        if let Some((path, _mime_type)) = self.doc.resources.get(&resolved_path) {
            // Clone the path to avoid borrowing issues
            let path_clone = path.clone();
            
            // Extract the resource data
            let data = self.doc.get_resource_by_path(&path_clone)
                .ok_or_else(|| format!("Failed to extract resource {}: data not found", resource_path))?;
            
            // Create a temporary file to store the resource
            let temp_dir = std::env::temp_dir();
            let path_buf = PathBuf::from(&resolved_path);
            let file_name = path_buf
                .file_name()
                .ok_or_else(|| "Invalid resource path".to_string())?
                .to_str()
                .ok_or_else(|| "Invalid resource path encoding".to_string())?;
            
            let temp_path = temp_dir.join(file_name);
            
            // Write the data to the temporary file
            std::fs::write(&temp_path, data)
                .map_err(|e| format!("Failed to write resource to temp file: {}", e))?;
            
            Ok(temp_path)
        } else {
            // Try to find the resource with a different approach
            // The resource path might be relative to the current chapter's path
            // Let's try to find any resource that ends with this path
            for key in resource_keys {
                if let Some((full_path, _mime_type)) = self.doc.resources.get(&key) {
                    if full_path.ends_with(&resolved_path) || full_path.ends_with(resource_path) {
                        // Clone the path to avoid borrowing issues
                        let path_clone = full_path.clone();
                        
                        // Extract the resource data
                        let data = self.doc.get_resource_by_path(&path_clone)
                            .ok_or_else(|| format!("Failed to extract resource {}: data not found", resource_path))?;
                        
                        // Create a temporary file to store the resource
                        let temp_dir = std::env::temp_dir();
                        let path_buf = PathBuf::from(resource_path);
                        let file_name = path_buf
                            .file_name()
                            .ok_or_else(|| "Invalid resource path".to_string())?
                            .to_str()
                            .ok_or_else(|| "Invalid resource path encoding".to_string())?;
                        
                        let temp_path = temp_dir.join(file_name);
                        
                        // Write the data to the temporary file
                        std::fs::write(&temp_path, data)
                            .map_err(|e| format!("Failed to write resource to temp file: {}", e))?;
                        
                        return Ok(temp_path);
                    }
                }
            }
            
            Err(format!("Resource not found: {} (resolved from {})", resolved_path, resource_path))
        }
    }
}
