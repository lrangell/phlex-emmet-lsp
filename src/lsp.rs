use async_lsp::lsp_types::{
    CompletionItem, CompletionItemKind, CompletionItemLabelDetails, CompletionOptions,
    CompletionResponse, Position, ServerCapabilities, TextDocumentSyncCapability,
    TextDocumentSyncKind,
};
use async_lsp::{ClientSocket, ResponseError};
use ropey::Rope;
use std::collections::HashMap;

pub struct ServerState {
    pub client: ClientSocket,
    pub documents: HashMap<String, Rope>,
}

pub fn capabilities() -> ServerCapabilities {
    ServerCapabilities {
        completion_provider: CompletionOptions::default().into(),
        text_document_sync: TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL).into(),
        ..ServerCapabilities::default()
    }
}

pub fn simple_completion_item() -> CompletionItem {
    CompletionItem {
        label: "label".into(),
        label_details: CompletionItemLabelDetails {
            detail: "details".to_string().into(),
            description: "description".to_string().into(),
        }
        .into(),
        kind: CompletionItemKind::SNIPPET.into(),
        detail: "details info".to_string().into(),
        insert_text: "text inset".to_string().into(),
        ..CompletionItem::default()
    }
}

pub async fn completion_handler(
    _document: Rope,
    _position: Position,
) -> Result<Option<CompletionResponse>, ResponseError> {
    let resp = CompletionResponse::Array(vec![simple_completion_item()]);
    Ok(Some(resp))
}
