use thiserror::Error;

#[derive(Debug, Error)]
pub enum CsvExportError {}

pub trait CsvExport {
    fn export_csv(&self) -> Result<String, CsvExportError>;
}
