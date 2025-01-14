use super::parser::parse;
use crate::rendering::Renderer;
use crate::types::EmmetNode;
use anyhow::Result;

use async_lsp::lsp_types::*;
use async_lsp::{ClientSocket, ResponseError};
use ropey::Rope;
use std::collections::HashMap;

pub struct ServerState {
    pub client: ClientSocket,
    pub documents: HashMap<String, Rope>,
}

pub fn capabilities() -> ServerCapabilities {
    let charset: Vec<String> = ('a'..='z')
        .chain('A'..='Z')
        .chain(['>', '.', '+', '*', '#'])
        .map(String::from)
        .collect();
    ServerCapabilities {
        completion_provider: CompletionOptions {
            trigger_characters: charset.into(),
            ..CompletionOptions::default()
        }
        .into(),
        text_document_sync: TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL).into(),
        ..ServerCapabilities::default()
    }
}

type CompletionResult = Result<Option<CompletionResponse>, ResponseError>;

fn text_edit(abbr: &str, expansion: String, position: Position) -> CompletionTextEdit {
    CompletionTextEdit::Edit(TextEdit {
        range: Range {
            start: Position {
                line: position.line,
                character: position.character - abbr.len() as u32,
            },
            end: Position {
                line: position.line,
                character: position.character,
            },
        },
        new_text: expansion,
    })
}

fn completion_item(abbr: &str, edit: CompletionTextEdit) -> CompletionItem {
    if let CompletionTextEdit::Edit(TextEdit { new_text, .. }) = &edit {
        let expansion = new_text;
        return CompletionItem {
            label: abbr.to_string(),
            kind: CompletionItemKind::SNIPPET.into(),
            detail: Some(expansion.clone()),
            documentation: None,
            text_edit: Some(edit),
            ..CompletionItem::default()
        };
    }
    unreachable!()
}

fn try_parse_completion(abbr: &str) -> Result<EmmetNode, ResponseError> {
    match parse(abbr) {
        Ok(node) => Ok(node),
        Err(_) => Err(ResponseError::new(
            async_lsp::ErrorCode::INTERNAL_ERROR,
            "Failed to parse Emmet expression",
        )),
    }
}

pub async fn completion_handler(abbr: &str, position: Position) -> CompletionResult {
    let Ok(expansion) = try_parse_completion(abbr).map(|e| e.render()) else {
        return Ok(None);
    };
    let edit = text_edit(abbr, expansion, position);
    let item = completion_item(abbr, edit);
    let response = CompletionResponse::Array(vec![item]);

    Ok(Some(response))
}
