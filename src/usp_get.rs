use std::error::Error;

// use crate::telemetry:*;
use crate::{
    protobuf::usp_msg::{
        body::MsgBody, header::MsgType, request::ReqType, Body, Get, Header, Msg, Request,
    },
    usp_agent::UspError,
    usp_msg_handle::UspMessageCreate,
};
// use std::sync::OnceLock;
// use tracing_subscriber::filter::LevelFilter;

pub struct USPGet<'a> {
    param_paths: Vec<String>,
    max_depth: u32,
    msg_id: &'a str,
}

pub struct USPGetBuildier<'a> {
    param_paths: Vec<String>,
    max_depth: u32,
    msg_id: Option<&'a String>,
}

// static TRACING: OnceLock<()> = OnceLock::new();
// fn tracing_init() {
//     let _tracing = TRACING.get_or_init(|| {
//         let test_sub = get_subscriber("test_debug".into(), LevelFilter::INFO.into());
//         init_subscriber(test_sub);
//     });
// }

impl<'a> USPGetBuildier<'a> {
    pub fn new(max_depth: u32, id: &'a String) -> Self {
        USPGetBuildier {
            param_paths: Vec::new(),
            max_depth,
            msg_id: Some(id),
        }
    }

    pub fn add_parameter_path(mut self, param_path: String) -> Self {
        self.param_paths.push(param_path);
        self
    }

    pub fn delete_parameter_path(mut self, index: usize) -> Self {
        self.param_paths.remove(index);
        self
    }

    pub fn build(self) -> Result<USPGet<'a>, UspError> {
        if let Some(msg_id) = self.msg_id {
            let usp_get = USPGet {
                param_paths: self.param_paths,
                max_depth: self.max_depth,
                msg_id,
            };
            Ok(usp_get)
        } else {
            Err(UspError::InvalidPathSyntax)
        }
    }
}

impl<'a> UspMessageCreate for USPGet<'a> {
    fn create_msg(&self) -> Msg {
        let msg_header = Header {
            msg_id: self.msg_id.into(),
            msg_type: MsgType::Get.into(),
        };
        let get_body = Get {
            param_paths: self.param_paths.iter().map(|x| x.to_string()).collect(),
            max_depth: self.max_depth,
        };
        let request = Request {
            req_type: Some(ReqType::Get(get_body)),
        };
        let msg_body = Body {
            msg_body: Some(MsgBody::Request(request)),
        };
        Msg {
            header: Some(msg_header),
            body: Some(msg_body),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::{usp_agent::UspAgent, usp_get::USPGet, usp_msg_handle::UspMessageCreate};
    // use tracing::ifo;
    #[test]
    fn get_la_success() {
        // tracing_init();
        let param_path = String::from("Device.LocalAgent.");
        let mut parameters: Vec<String> = Vec::new();
        parameters.push(param_path);
        let get_req = USPGet {
            param_paths: &parameters,
            msg_id: "0",
            max_depth: 5,
        };
        let msg = get_req.create_msg();
        let agent = UspAgent::new(String::from("ops::DarwinTran"));
        let record_get = agent.create_record(&msg, &String::from("ops::agent1"));
    }
}
