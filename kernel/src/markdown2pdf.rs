use markdown2pdf::{config::ConfigSource, parse_into_file};

use crate::error::KernelError;

// Default styling — saves the PDF to the user's Downloads directory
pub fn parse_markdown_to_pdf(markdown: &str, file_name: &str) -> Result<(), KernelError> {
    let output_path = dirs::download_dir()
        .ok_or(KernelError::DownloadDirNotFound)?
        .join(file_name);

    parse_into_file(
        markdown.to_string(),
        &output_path.to_string_lossy(),
        ConfigSource::Default,
        None,
    )
    .map_err(|e| KernelError::Markdown2Pdf(e.to_string()))
}
