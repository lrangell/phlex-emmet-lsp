use std::ops::ControlFlow;

use async_lsp::client_monitor::ClientProcessMonitorLayer;
use async_lsp::concurrency::ConcurrencyLayer;
use async_lsp::lsp_types::{notification, request, InitializeResult, ServerInfo};
use async_lsp::panic::CatchUnwindLayer;
use async_lsp::router::Router;
use async_lsp::server::LifecycleLayer;
use async_lsp::tracing::TracingLayer;
use async_lsp::ErrorCode;
use ropey::Rope;
use std::collections::HashMap;
use std::fs::OpenOptions;
use tower::ServiceBuilder;
use tracing::info;
use utils::extract_abbreviation;

mod lsp;
mod parser;
mod rendering;
mod types;
mod utils;

struct TickEvent;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let log_path = std::env::temp_dir().join("phlex_emmet_ls").join("logs");
    let _ = std::fs::create_dir(log_path.parent().unwrap());
    let file = OpenOptions::new()
        .read(true)
        .create(true)
        .truncate(true)
        .write(true)
        .open(log_path)
        .unwrap();
    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_writer(file)
        .init();

    info!("Starting LSP server");
    let (server, _) = async_lsp::MainLoop::new_server(|client| {
        let mut router = Router::new(lsp::ServerState {
            client: client.clone(),
            documents: HashMap::default(),
        });
        router
            .request::<request::Initialize, _>(|_, _| async move {
                Ok(InitializeResult {
                    capabilities: lsp::capabilities(),
                    server_info: ServerInfo {
                        name: "Phlex Emmet Language Server".to_string(),
                        version: env!("CARGO_PKG_VERSION").to_string().into(),
                    }
                    .into(),
                })
            })
            .request::<request::Completion, _>(|st, params| {
                let uri = params
                    .clone()
                    .text_document_position
                    .clone()
                    .text_document
                    .uri
                    .as_str()
                    .to_owned();

                let doc = st
                    .documents
                    .get(&uri)
                    .ok_or(async_lsp::ResponseError::new(
                        ErrorCode::INTERNAL_ERROR,
                        "Document not found",
                    ))
                    .unwrap();
                let abbr = extract_abbreviation(doc, params.text_document_position.position);
                async move {
                    Ok(lsp::completion_handler(
                        abbr.as_str(),
                        params.text_document_position.position,
                    )
                    .await?)
                }
            })
            .notification::<notification::Initialized>(|_, _| ControlFlow::Continue(()))
            .notification::<notification::DidChangeConfiguration>(|_, _| ControlFlow::Continue(()))
            .notification::<notification::DidOpenTextDocument>(|_, _| ControlFlow::Continue(()))
            .notification::<notification::DidChangeTextDocument>(|_, _| ControlFlow::Continue(()))
            .notification::<notification::DidChangeTextDocument>(|st, params| {
                let text = &params.content_changes.first().unwrap().text;
                let uri = params.text_document.uri.clone().as_str().to_owned();
                let document = Rope::from_str(text);
                st.documents.insert(uri, document);

                ControlFlow::Continue(())
            })
            .notification::<notification::DidSaveTextDocument>(|_, _| ControlFlow::Continue(()))
            .event::<TickEvent>(|_, _| ControlFlow::Continue(()));

        ServiceBuilder::new()
            .layer(TracingLayer::default())
            .layer(LifecycleLayer::default())
            .layer(CatchUnwindLayer::default())
            .layer(ConcurrencyLayer::default())
            .layer(ClientProcessMonitorLayer::new(client))
            .service(router)
    });

    // Prefer truly asynchronous piped stdin/stdout without blocking tasks.
    #[cfg(unix)]
    let (stdin, stdout) = (
        async_lsp::stdio::PipeStdin::lock_tokio().unwrap(),
        async_lsp::stdio::PipeStdout::lock_tokio().unwrap(),
    );
    // Fallback to spawn blocking read/write otherwise.
    #[cfg(not(unix))]
    let (stdin, stdout) = (
        tokio_util::compat::TokioAsyncReadCompatExt::compat(tokio::io::stdin()),
        tokio_util::compat::TokioAsyncWriteCompatExt::compat_write(tokio::io::stdout()),
    );

    server.run_buffered(stdin, stdout).await.unwrap();
}

#[cfg(test)]
mod tests;
