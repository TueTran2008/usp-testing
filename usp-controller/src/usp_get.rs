use crate::{protobuf::usp_msg::Msg, usp_msg_handle::UspMessageCreate};
use usp_msg::{body::MsgBody, header::MsgType, request::ReqType, Body, Get, Header, Msg, Request};
pub mod usp_msg {
    include!(concat!(env!("OUT_DIR"), "/usp.rs"));
    include!(concat!(env!("OUT_DIR"), "/usp_record.rs"));
}

pub struct USPGet {
    param_paths: Vec<String>,
    max_depth: u32,
    msg_id: String,
}

impl UspMessageCreate for USPGet {
    fn create_msg(&self) -> Msg {
        let msg_header = Header {
            msg_id: self.msg_id.clone(),
            msg_type: MsgType::Get.into(),
        };
        let get_body = Get {
            param_paths: self.param_paths.clone(),
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
