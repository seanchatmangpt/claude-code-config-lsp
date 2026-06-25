use lsp_max::jsonrpc::{Error, Result};
use lsp_max::lsp_types::request::{
    GotoDeclarationParams, GotoDeclarationResponse, GotoImplementationParams,
    GotoImplementationResponse, GotoTypeDefinitionParams, GotoTypeDefinitionResponse,
};
use lsp_max::lsp_types::*;
use lsp_max::max_protocol::{LawAxis, MaxDiagnostic};
use lsp_max::{
    ClassifiedFindings, Client, Finding, LanguageServer, RulePackServer, ValidatedRulePackSet,
    WorkspaceIndex,
};
use std::sync::{Arc, Mutex};

/// ConfigBackend is the LSP server implementation for Claude Code configuration files.
/// It analyzes settings.json, settings.local.json, keybindings.json, and other config surfaces
/// to detect schema violations, illegal settings, hook misconfigurations, and semantic issues.
pub struct ConfigBackend {
    pub client: Client,
    pub workspace_root: Arc<Mutex<Option<String>>>,
    pub workspace_index: WorkspaceIndex,
    rule_packs: ValidatedRulePackSet,
    adapter: lsp_max::ast::AutoLspAdapter,
}

impl ConfigBackend {
    pub fn new(client: Client) -> Self {
        Self {
            client,
            workspace_root: Arc::new(Mutex::new(None)),
            workspace_index: WorkspaceIndex::new(),
            rule_packs: ValidatedRulePackSet::empty(),
            adapter: lsp_max::ast::AutoLspAdapter::new_default(),
        }
    }

    fn _root_dir(&self) -> String {
        let guard = self.workspace_root.lock().unwrap();
        guard.clone().unwrap_or_else(|| ".".to_string())
    }

    /// Analyze a configuration file URI and return diagnostics.
    fn file_diagnostics(&self, uri: &Uri) -> Vec<Diagnostic> {
        let norm_uri = uri.to_string().replace('\\', "/");

        if norm_uri.contains("settings.json") || norm_uri.ends_with(".json") {
            self.analyze_json_config(&norm_uri)
        } else if norm_uri.contains("keybindings.json") {
            self.analyze_keybindings(&norm_uri)
        } else {
            vec![]
        }
    }

    fn analyze_json_config(&self, _uri: &str) -> Vec<Diagnostic> {
        vec![]
    }

    fn analyze_keybindings(&self, _uri: &str) -> Vec<Diagnostic> {
        vec![]
    }

    async fn run_scan_and_publish(&self, uri: &Uri) {
        let file_diags = self.file_diagnostics(uri);
        self.client
            .publish_diagnostics(uri.clone(), file_diags, None)
            .await;
        self.fire_refreshes(uri).await;
    }

    async fn fire_refreshes(&self, uri: &Uri) {
        let _ = self.client.code_lens_refresh().await;
        let _ = self.client.semantic_tokens_refresh().await;
        let _ = self.client.inlay_hint_refresh().await;
        let _ = self.client.inline_value_refresh().await;
        let _ = self
            .client
            .text_document_content_refresh(
                lsp_max::max_protocol::lsp_3_18::TextDocumentContentRefreshParams {
                    uri: uri.to_string(),
                },
            )
            .await;
        self.client
            .telemetry_event(serde_json::json!({"scan": "PARTIAL"}))
            .await;
    }
}

#[lsp_max::async_trait]
impl RulePackServer for ConfigBackend {
    fn rule_packs(&self) -> &ValidatedRulePackSet {
        &self.rule_packs
    }

    fn grammar(&self) -> tree_sitter::Language {
        // JSON grammar not yet wired; use Rust as placeholder — parser is not exercised
        // for config files since we bypass the AST path via scan_uri_classified override.
        tree_sitter_rust::LANGUAGE.into()
    }

    fn server_name(&self) -> &'static str {
        "claude-code-config-lsp"
    }

    fn client(&self) -> &Client {
        &self.client
    }

    fn adapter(&self) -> &lsp_max::ast::AutoLspAdapter {
        &self.adapter
    }

    fn workspace_index(&self) -> Option<&WorkspaceIndex> {
        Some(&self.workspace_index)
    }

    fn scan_uri_classified(&self, uri: &DocumentUri, _content: &str) -> ClassifiedFindings {
        let findings: Vec<Finding> = self
            .file_diagnostics(uri)
            .into_iter()
            .map(|lsp_diag| {
                let max_diag = MaxDiagnostic {
                    lsp: lsp_diag.clone(),
                    law_axis: LawAxis::Domain,
                    ..MaxDiagnostic::default()
                };
                (max_diag, lsp_diag)
            })
            .collect();

        (findings, vec![])
    }
}

#[lsp_max::async_trait]
impl LanguageServer for ConfigBackend {
    // ════════════════════════════════════════════════════════════════════════════════
    // Lifecycle Methods
    // ════════════════════════════════════════════════════════════════════════════════

    #[allow(deprecated, clippy::field_reassign_with_default)]
    async fn initialize(&self, params: InitializeParams) -> Result<InitializeResult> {
        if let Some(uri) = params.root_uri {
            if let Ok(url) = url::Url::parse(uri.as_str()) {
                if let Ok(path) = url.to_file_path() {
                    let mut root = self.workspace_root.lock().unwrap();
                    *root = Some(path.to_string_lossy().to_string());
                }
            }
        }

        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::INCREMENTAL,
                )),
                diagnostic_provider: Some(DiagnosticServerCapabilities::Options(
                    DiagnosticOptions {
                        work_done_progress_options: WorkDoneProgressOptions::default(),
                        ..Default::default()
                    },
                )),
                ..Default::default()
            },
            server_info: Some(ServerInfo {
                name: "claude-code-config-lsp".to_string(),
                version: Some("26.6.24".to_string()),
            }),
            offset_encoding: None,
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "claude-code-config-lsp initialized")
            .await;

        self.client
            .show_message(MessageType::INFO, "Configuration analysis active")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_change_configuration(&self, params: DidChangeConfigurationParams) {
        self.client
            .log_message(
                MessageType::INFO,
                format!("Configuration changed: {:?}", params.settings),
            )
            .await;
    }

    // ════════════════════════════════════════════════════════════════════════════════
    // Text Document Synchronization Methods
    // ════════════════════════════════════════════════════════════════════════════════

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = &params.text_document.uri;
        self.workspace_index.upsert(
            uri.to_string(),
            params.text_document.text,
            params.text_document.version,
        );
        self.run_scan_and_publish(uri).await;
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = &params.text_document.uri;
        let indexed = self.workspace_index.get(&uri.to_string());
        let mut content = indexed
            .map(|doc| doc.content.clone())
            .unwrap_or_default();

        for change in params.content_changes {
            if let Some(range) = change.range {
                let mut line_start = 0;
                let mut col_start = 0;
                for (i, line) in content.lines().enumerate() {
                    if i == range.start.line as usize {
                        col_start = range.start.character as usize;
                        break;
                    }
                    line_start += line.len() + 1;
                }

                let start = line_start + col_start;
                let mut line_end = 0;
                let mut col_end = 0;
                for (i, line) in content.lines().enumerate() {
                    if i == range.end.line as usize {
                        col_end = range.end.character as usize;
                        break;
                    }
                    line_end += line.len() + 1;
                }
                let end = line_end + col_end;

                if start <= content.len() && end <= content.len() {
                    content.replace_range(start..end, &change.text);
                }
            } else {
                content = change.text;
            }
        }

        self.workspace_index.upsert(
            uri.to_string(),
            content,
            params.text_document.version,
        );
        self.run_scan_and_publish(uri).await;
    }

    async fn did_close(&self, params: DidCloseTextDocumentParams) {
        let uri = params.text_document.uri.to_string();
        self.workspace_index.remove(&uri);
        self.client
            .publish_diagnostics(params.text_document.uri, vec![], None)
            .await;
    }

    async fn did_save(&self, params: DidSaveTextDocumentParams) {
        let uri = &params.text_document.uri;
        self.run_scan_and_publish(uri).await;
    }

    async fn will_save(&self, _params: WillSaveTextDocumentParams) {}

    async fn will_save_wait_until(
        &self,
        _params: WillSaveTextDocumentParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        Ok(None)
    }

    // ════════════════════════════════════════════════════════════════════════════════
    // Diagnostic Methods
    // ════════════════════════════════════════════════════════════════════════════════

    async fn diagnostic(
        &self,
        params: DocumentDiagnosticParams,
    ) -> Result<DocumentDiagnosticReportResult> {
        let uri = &params.text_document.uri;
        let diags = self.file_diagnostics(uri);

        Ok(DocumentDiagnosticReportResult::Report(
            DocumentDiagnosticReport::Full(RelatedFullDocumentDiagnosticReport {
                related_documents: None,
                full_document_diagnostic_report: FullDocumentDiagnosticReport {
                    result_id: None,
                    items: diags,
                },
            }),
        ))
    }

    // ════════════════════════════════════════════════════════════════════════════════
    // Semantic Tokens Methods
    // ════════════════════════════════════════════════════════════════════════════════

    async fn semantic_tokens_full(
        &self,
        _params: SemanticTokensParams,
    ) -> Result<Option<SemanticTokensResult>> {
        Ok(None)
    }

    async fn semantic_tokens_full_delta(
        &self,
        _params: SemanticTokensDeltaParams,
    ) -> Result<Option<SemanticTokensFullDeltaResult>> {
        Ok(None)
    }

    // ════════════════════════════════════════════════════════════════════════════════
    // Hover, Completion, Resolution Methods
    // ════════════════════════════════════════════════════════════════════════════════

    async fn hover(&self, _params: HoverParams) -> Result<Option<Hover>> {
        Ok(None)
    }

    async fn completion(&self, _params: CompletionParams) -> Result<Option<CompletionResponse>> {
        Ok(None)
    }

    async fn completion_resolve(&self, item: CompletionItem) -> Result<CompletionItem> {
        Ok(item)
    }

    async fn signature_help(
        &self,
        _params: SignatureHelpParams,
    ) -> Result<Option<SignatureHelp>> {
        Ok(None)
    }

    async fn goto_declaration(
        &self,
        _params: GotoDeclarationParams,
    ) -> Result<Option<GotoDeclarationResponse>> {
        Ok(None)
    }

    async fn goto_definition(
        &self,
        _params: GotoDefinitionParams,
    ) -> Result<Option<GotoDefinitionResponse>> {
        Ok(None)
    }

    // ════════════════════════════════════════════════════════════════════════════════
    // Reference, Implementation, Type Definition Methods
    // ════════════════════════════════════════════════════════════════════════════════

    async fn references(&self, _params: ReferenceParams) -> Result<Option<Vec<Location>>> {
        Ok(None)
    }

    async fn goto_implementation(
        &self,
        _params: GotoImplementationParams,
    ) -> Result<Option<GotoImplementationResponse>> {
        Ok(None)
    }

    async fn goto_type_definition(
        &self,
        _params: GotoTypeDefinitionParams,
    ) -> Result<Option<GotoTypeDefinitionResponse>> {
        Ok(None)
    }

    // ════════════════════════════════════════════════════════════════════════════════
    // Symbol Methods
    // ════════════════════════════════════════════════════════════════════════════════

    async fn document_symbol(
        &self,
        _params: DocumentSymbolParams,
    ) -> Result<Option<DocumentSymbolResponse>> {
        Ok(None)
    }

    async fn symbol(
        &self,
        _params: WorkspaceSymbolParams,
    ) -> Result<Option<Vec<SymbolInformation>>> {
        Ok(None)
    }

    async fn symbol_resolve(&self, symbol: WorkspaceSymbol) -> Result<WorkspaceSymbol> {
        Ok(symbol)
    }

    // ════════════════════════════════════════════════════════════════════════════════
    // Code Lens, Code Action, Formatting Methods
    // ════════════════════════════════════════════════════════════════════════════════

    async fn code_lens(&self, _params: CodeLensParams) -> Result<Option<Vec<CodeLens>>> {
        Ok(None)
    }

    async fn code_lens_resolve(&self, lens: CodeLens) -> Result<CodeLens> {
        Ok(lens)
    }

    async fn code_action(
        &self,
        _params: CodeActionParams,
    ) -> Result<Option<CodeActionResponse>> {
        Ok(None)
    }

    async fn code_action_resolve(&self, action: CodeAction) -> Result<CodeAction> {
        Ok(action)
    }

    async fn formatting(
        &self,
        _params: DocumentFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        Ok(None)
    }

    async fn range_formatting(
        &self,
        _params: DocumentRangeFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        Ok(None)
    }

    // ════════════════════════════════════════════════════════════════════════════════
    // Folding Range, Selection Range, Inlay Hint Methods
    // ════════════════════════════════════════════════════════════════════════════════

    async fn folding_range(
        &self,
        _params: FoldingRangeParams,
    ) -> Result<Option<Vec<FoldingRange>>> {
        Ok(None)
    }

    async fn selection_range(
        &self,
        _params: SelectionRangeParams,
    ) -> Result<Option<Vec<SelectionRange>>> {
        Ok(None)
    }

    async fn inlay_hint(
        &self,
        _params: InlayHintParams,
    ) -> Result<Option<Vec<InlayHint>>> {
        Ok(None)
    }

    async fn inlay_hint_resolve(&self, hint: InlayHint) -> Result<InlayHint> {
        Ok(hint)
    }

    async fn inline_value(
        &self,
        _params: InlineValueParams,
    ) -> Result<Option<Vec<InlineValue>>> {
        Ok(None)
    }

    // ════════════════════════════════════════════════════════════════════════════════
    // Call Hierarchy, Linked Editing, On Type Formatting Methods
    // ════════════════════════════════════════════════════════════════════════════════

    async fn prepare_call_hierarchy(
        &self,
        _params: CallHierarchyPrepareParams,
    ) -> Result<Option<Vec<CallHierarchyItem>>> {
        Ok(None)
    }

    async fn incoming_calls(
        &self,
        _params: CallHierarchyIncomingCallsParams,
    ) -> Result<Option<Vec<CallHierarchyIncomingCall>>> {
        Ok(None)
    }

    async fn outgoing_calls(
        &self,
        _params: CallHierarchyOutgoingCallsParams,
    ) -> Result<Option<Vec<CallHierarchyOutgoingCall>>> {
        Ok(None)
    }

    async fn prepare_type_hierarchy(
        &self,
        _params: TypeHierarchyPrepareParams,
    ) -> Result<Option<Vec<TypeHierarchyItem>>> {
        Ok(None)
    }

    async fn on_type_formatting(
        &self,
        _params: DocumentOnTypeFormattingParams,
    ) -> Result<Option<Vec<TextEdit>>> {
        Ok(None)
    }

    // ════════════════════════════════════════════════════════════════════════════════
    // Linked Editing, Type Hierarchy, Moniker Methods
    // ════════════════════════════════════════════════════════════════════════════════

    async fn linked_editing_range(
        &self,
        _params: LinkedEditingRangeParams,
    ) -> Result<Option<LinkedEditingRanges>> {
        Ok(None)
    }

    async fn supertypes(
        &self,
        _params: TypeHierarchySupertypesParams,
    ) -> Result<Option<Vec<TypeHierarchyItem>>> {
        Ok(None)
    }

    async fn moniker(&self, _params: MonikerParams) -> Result<Option<Vec<Moniker>>> {
        Ok(None)
    }

    // ════════════════════════════════════════════════════════════════════════════════
    // Document Link, Rename, Workspace Methods
    // ════════════════════════════════════════════════════════════════════════════════

    async fn document_link(
        &self,
        _params: DocumentLinkParams,
    ) -> Result<Option<Vec<DocumentLink>>> {
        Ok(None)
    }

    async fn document_link_resolve(&self, link: DocumentLink) -> Result<DocumentLink> {
        Ok(link)
    }

    async fn rename(&self, _params: RenameParams) -> Result<Option<WorkspaceEdit>> {
        Ok(None)
    }

    async fn prepare_rename(
        &self,
        _params: TextDocumentPositionParams,
    ) -> Result<Option<PrepareRenameResponse>> {
        Ok(None)
    }

    // ════════════════════════════════════════════════════════════════════════════════
    // Workspace Folder Methods
    // ════════════════════════════════════════════════════════════════════════════════

    async fn did_change_workspace_folders(&self, _params: DidChangeWorkspaceFoldersParams) {}

    async fn execute_command(
        &self,
        _params: ExecuteCommandParams,
    ) -> Result<Option<serde_json::Value>> {
        Ok(None)
    }

    // ════════════════════════════════════════════════════════════════════════════════
    // Document Color Methods
    // ════════════════════════════════════════════════════════════════════════════════

    async fn document_color(
        &self,
        _params: DocumentColorParams,
    ) -> Result<Vec<ColorInformation>> {
        Ok(vec![])
    }

    async fn color_presentation(
        &self,
        _params: ColorPresentationParams,
    ) -> Result<Vec<ColorPresentation>> {
        Ok(vec![])
    }

    // ════════════════════════════════════════════════════════════════════════════════
    // Text Document Content Provider
    // ════════════════════════════════════════════════════════════════════════════════

    async fn text_document_content(
        &self,
        _params: lsp_max::max_protocol::lsp_3_18::TextDocumentContentParams,
    ) -> Result<lsp_max::max_protocol::lsp_3_18::TextDocumentContentResult> {
        Err(Error::method_not_found())
    }
}
