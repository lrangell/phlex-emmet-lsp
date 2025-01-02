use async_lsp::lsp_types::Position;
use ropey::Rope;
use tracing::info;

pub fn extract_abbreviation(doc: &Rope, position: Position) -> String {
    let line_number = position.line as usize;
    let col_number = position.character as usize;
    let line = doc.line(line_number).to_string();
    let line = &line[..col_number];
    let start = line.rfind([' ', '\t', '{']).map(|i| i + 1).unwrap_or(0);
    let abbr = &line[start..];
    info!("Abbreviation: {:#?}", abbr);

    abbr.to_string()
}
