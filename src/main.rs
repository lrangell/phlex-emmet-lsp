use std::ops::ControlFlow;

use async_lsp::client_monitor::ClientProcessMonitorLayer;
use async_lsp::concurrency::ConcurrencyLayer;
use async_lsp::lsp_types::{notification, request, InitializeResult};
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

mod lsp;
mod parser;
mod rendering;
mod types;

struct TickEvent;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let file = OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open("logs")
        .unwrap();
    info!("Starting LSP server");
    let (server, _) = async_lsp::MainLoop::new_server(|client| {
        info!("server");

        let mut router = Router::new(lsp::ServerState {
            client: client.clone(),
            documents: HashMap::default(),
        });
        router
            .request::<request::Initialize, _>(|_, _| async move {
                Ok(InitializeResult {
                    capabilities: lsp::capabilities(),
                    server_info: None,
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
                let line_number = params.text_document_position.position.line as usize;
                let col_number = params.text_document_position.position.character as usize;

                let doc = st
                    .documents
                    .get(&uri)
                    .ok_or(async_lsp::ResponseError::new(
                        ErrorCode::INTERNAL_ERROR,
                        "Document not found",
                    ))
                    .unwrap();
                let line = doc.line(line_number);
                let text = line.slice(..col_number).to_string();
                info!("Completion request: {:#?}", &text);
                async move { Ok(lsp::completion_handler(text).await?) }
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

    tracing_subscriber::fmt()
        .with_ansi(true)
        .with_writer(file)
        .init();

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
