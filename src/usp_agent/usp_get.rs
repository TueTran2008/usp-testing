use crate::error::{Error, Result};
use crate::usp_agent::uspa::UspError;
use std::string::String;
#[derive(Debug)]
pub struct USPGet {
    param_paths: Vec<String>,
    max_depth: u32,
    msg_id: String,
}

#[derive(Default, Clone)]
pub struct USPGetBuilder {
    param_paths: Vec<String>,
    max_depth: Option<u32>,
    msg_id: Option<String>,
}

impl USPGetBuilder {
    pub fn new() -> Self {
        USPGetBuilder::default()
    }
    pub fn param_path(mut self, path: impl Into<String>) -> Self {
        self.param_paths.push(path.into());
        self
    }
    pub fn max_depth(mut self, depth: u32) -> Self {
        self.max_depth.insert(depth);
        self
    }
    pub fn msg_id(mut self, msg: impl Into<String>) -> Self {
        self.msg_id.insert(msg.into());
        self
    }
    pub fn build(self) -> Result<USPGet> {
        if self.param_paths.is_empty() {
            return Err(Error::UspAgentError(UspError::InvalidPath));
        };
        let Some(msg_id) = self.msg_id else {
            return Err(Error::UspAgentError(UspError::GeneralFailure));
        };
        let max_depth = self.max_depth.unwrap_or_else(|| 5);
        Ok(USPGet {
            param_paths: self.param_paths.clone(),
            msg_id: msg_id.clone(),
            max_depth,
        })
    }
}
