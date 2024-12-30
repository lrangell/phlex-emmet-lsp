use super::parser::parse;
use crate::rendering::Renderer;
use anyhow::Result;

use async_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, CompletionItemLabelDetails, CompletionOptions,
    CompletionResponse, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind,
};
use async_lsp::{ClientSocket, ResponseError};
use ropey::Rope;
use std::collections::HashMap;
use tracing::info;

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
    info!(charset = ?charset, "capabilities");
    ServerCapabilities {
        completion_provider: CompletionOptions {
            trigger_characters: charset.into(),
            resolve_provider: Some(true),
            ..CompletionOptions::default()
        }
        .into(),
        text_document_sync: TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL).into(),
        ..ServerCapabilities::default()
    }
}

pub fn simple_completion_item(text: String, expansion: String) -> CompletionItem {
    info!("expansion: {}", expansion);
    CompletionItem {
        label: text.clone(),
        label_details: CompletionItemLabelDetails {
            detail: "label_details".to_string().into(),
            description: "description".to_string().into(),
        }
        .into(),
        kind: CompletionItemKind::SNIPPET.into(),
        detail: Some(expansion.clone()),
        insert_text: Some(expansion),
        ..CompletionItem::default()
    }
}

fn completion_from_input(text: String) -> Result<CompletionItem> {
    let expansion = parse(text.clone())?.render();
    info!(text = text, expansion = expansion, "completion_from_input");
    Ok(simple_completion_item(text, expansion))
}

type CompletionResult = Result<Option<CompletionResponse>, ResponseError>;

fn internal_error(message: &str) -> CompletionResult {
    Err(ResponseError::new(
        async_lsp::ErrorCode::INTERNAL_ERROR,
        message,
    ))
}

pub async fn completion_handler(text: String) -> CompletionResult {
    match completion_from_input(text) {
        Ok(item) => Ok(Some(CompletionResponse::List(
            async_lsp::lsp_types::CompletionList {
                is_incomplete: false,
                items: vec![item],
            },
        ))),
        Err(e) => internal_error(&e.to_string()),
    }
}
