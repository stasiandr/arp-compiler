use arp_types::{errors::DiagnosticError, sources::Source};
use tower_lsp::{lsp_types::{Diagnostic, DidChangeTextDocumentParams, DidCloseTextDocumentParams, DidOpenTextDocumentParams, DidSaveTextDocumentParams, InitializeParams, InitializeResult, InitializedParams, MessageType, Position, Range, ServerCapabilities, TextDocumentSyncCapability, TextDocumentSyncKind, Url}, Client, LanguageServer, LspService};
use tower_lsp::jsonrpc::Result;
use tower_lsp::Server;


#[derive(Debug)]
struct ArpBackend {
    client: Client,
}


struct TextDocumentItem {
    uri: Url,
    text: String,

    #[allow(dead_code)]
    version: i32,
}


#[tower_lsp::async_trait]
impl LanguageServer for ArpBackend {

    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(TextDocumentSyncKind::FULL)),

                ..Default::default()
            },




            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized!")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let _ = params;
        
        self.client
            .log_message(MessageType::INFO, "file did save!")
            .await;
    }


    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let _ = params;
        
        self.client
            .log_message(MessageType::INFO, "file did open!")
            .await;
        
        // self.run_diagnostics(TextDocumentItem {
        //     uri: params.text_document.uri,
        //     text: params.text_document.text,
        //     version: params.text_document.version,
        // })
        // .await
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let _ = params;
        
        self.client
            .log_message(MessageType::INFO, "file did close!")
            .await;
    }

    async fn did_change(&self, mut params: DidChangeTextDocumentParams) {
        let _ = params;

        self.client
            .log_message(MessageType::INFO, "file did change!")
            .await;

        self.run_diagnostics(TextDocumentItem {
            uri: params.text_document.uri,
            text: std::mem::take(&mut params.content_changes[0].text),
            version: params.text_document.version,
        }).await;
    }
}

impl ArpBackend {
    async fn run_diagnostics(&self, doc: TextDocumentItem) {
        let source = Source::new_inline(doc.uri.path(), doc.text);

        let diags = parse_source_for_errors(&source);

        self.client
            .log_message(MessageType::INFO, format!("{:?}", diags.len()))
            .await;

        let diags = diags.iter().flat_map(|d| convert(source.content(), d)).collect();

        self.client.publish_diagnostics(doc.uri, diags, None).await;
    }
}

fn convert(input: &str, error: &DiagnosticError) -> Option<Diagnostic> {
    Some(Diagnostic::new_simple(convert_span(input, error.range())?, error.message().to_owned()))
}

fn convert_span(input: &str, range: &std::ops::Range<usize>) -> Option<Range> {
    let (start_line, start_pos) = get_line_and_position(input, range.start)?;
    let (end_line, end_pos) = get_line_and_position(input, range.end)?;
    Some(Range { 
        start: Position {
            line: start_line as u32,
            character: start_pos as u32,
        }, 
        end: Position {
            line: end_line as u32,
            character: end_pos as u32,
        },  
    })
}

fn parse_source_for_errors(source: &Source) -> Vec<DiagnosticError> {
    match arp_lexer::lex_tokens(source) {
        Ok(input) => {
            match arp_parser::parse_arp_file(source.len(), &input) {
                Ok(_ast) => {
                    vec![]
                },
                Err(errors) => {
                    errors.iter().flat_map(|e| e.clone().try_into()).collect()
                },
            }
        },
        Err(errors) => errors.iter().flat_map(|e| e.clone().try_into()).collect(),
    }
}




#[tokio::main]
pub async fn run_server() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, socket) = LspService::build(|client| ArpBackend {
        client,
    })
    .finish();

    Server::new(stdin, stdout, socket).serve(service).await;
}

fn get_line_and_position(text: &str, offset: usize) -> Option<(usize, usize)> {
    let mut cumulative_count = 0;

    for (line_number, line) in text.lines().enumerate() {
        let line_length = line.len() + 1;

        if cumulative_count + line_length > offset {
            let position_in_line = offset - cumulative_count;
            return Some((line_number, position_in_line));
        }

        cumulative_count += line_length;
    }

    None
}