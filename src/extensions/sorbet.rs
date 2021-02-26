use crate::language_client::LanguageClient;
use anyhow::Result;
use jsonrpc_core::Value;
use log::info;
use lsp_types::request::Request;

pub mod request {
    use lsp_types::{request::Request, TextDocumentIdentifier, TextDocumentItem};
    pub enum SorbetReadFile {}

    impl Request for SorbetReadFile {
        type Params = TextDocumentIdentifier;
        type Result = TextDocumentItem;
        const METHOD: &'static str = "sorbet/readFile";
    }
}

impl LanguageClient {
    pub fn sorbet_read_file(&self, params: &Value) -> Result<Value> {
        info!("Begin {}", request::SorbetReadFile::METHOD);

        let result = self
            .get_client(&Some("ruby".to_string()))?
            .call(request::SorbetReadFile::METHOD, params)?;

        info!("End {}", request::SorbetReadFile::METHOD);

        Ok(result)
    }
}

// // Sorbet (sorbet.org) returns `sorbet:` URIs when it knows about a file but that file does not
// // exist on the client's disk. That happens in two situations:
// // - The file is a part of the standard library RBI type annotations, which comes baked into
// //   the Sorbet binary as a string.
// // - The client has passed Sorbet's --lsp-directories-missing-from-client flag, which declares
// //   folders that only exist where the server is running (say, a remote development machine),
// //   not where the editor is running (a laptop talking to that machine over SSH).
// // In both cases, Sorbet supports a `sorbet/readFile` file operation that takes in `sorbet:...`
// // URIs and returns the text content of the file (among other things).
// //
// // Implementation of sorbet/readFile in Sorbet:
// // https://github.com/sorbet/sorbet/blob/master/main/lsp/requests/sorbet_read_file.cc
// pub fn sorbet_read_file(&self, params: &Value) -> Result<Value> {
//     info!("Begin {}", REQUEST_SORBET_READ_FILE);
//     let language_id = "ruby".to_string();

//     let content: Value = self
//         .get_client(&Some(language_id))?
//         .call(REQUEST_SORBET_READ_FILE, params)?;

//     let content: String =
//         try_get("text", &content)?.ok_or_else(|| anyhow!("text not found in response!"))?;

//     let lines: Vec<String> = content
//         .lines()
//         .map(std::string::ToString::to_string)
//         .collect();

//     let goto_cmd = self
//         .vim()?
//         .get_goto_cmd(params)?
//         .unwrap_or_else(|| "edit".to_string());

//     let uri: String =
//         try_get("uri", params)?.ok_or_else(|| anyhow!("uri not found in request!"))?;

//     self.vim()?
//         .rpcclient
//         .notify("s:Edit", json!([goto_cmd, uri]))?;

//     self.vim()?.setline(1, &lines)?;
//     self.vim()?
//         .command("setlocal buftype=nofile filetype=ruby noswapfile")?;

//     info!("End {}", REQUEST_SORBET_READ_FILE);
//     Ok(Value::String(content))
// }
