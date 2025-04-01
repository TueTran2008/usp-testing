use crate::protobuf::usp_msg::set::UpdateObject;
use crate::telemetry::*;
use crate::{
    protobuf::usp_msg::{
        body::MsgBody, header::MsgType, request::ReqType, Body, Header, Msg, Request, Set,
    },
    usp_msg_handle::UspMessageCreate,
};
// use std::sync::OnceLock;
use tracing_subscriber::filter::LevelFilter;

pub struct USPSet<'a> {
    allow_partial: bool,
    update_object: &'a Vec<UpdateObject>,
    msg_id: &'a String,
}

// static TRACING: OnceLock<()> = OnceLock::new();
// fn tracing_init() {
//     assert!(TRACING.get().is_none());
//     let _tracing = TRACING.get_or_init(|| {
//         let test_sub = get_subscriber("test_debug".into(), LevelFilter::INFO.into());
//         init_subscriber(test_sub);
//     });
//     assert!(TRACING.get().is_none());
// }

impl<'a> USPSet<'a> {
    pub fn new_update_object() {}
}
impl<'a> UspMessageCreate for USPSet<'a> {
    fn create_msg(&self) -> Msg {
        let msg_header = Header {
            msg_id: self.msg_id.into(),
            msg_type: MsgType::Get.into(),
        };
        let set_body = Set {
            allow_partial: self.allow_partial,
            update_objs: self.update_object.to_vec(),
        };
        let request = Request {
            req_type: Some(ReqType::Set(set_body)),
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

// #[cfg(test)]
// mod test {
//     use super::*;
//     use crate::{usp_agent::UspAgent, usp_get::USPGet, usp_msg_handle::UspMessageCreate};
//     fn get_la_success() {
//         // tracing_init();
//         // let param_path = String::from("Device.LocalAgent.");
//         // let mut parameters: Vec<String> = Vec::new();
//         // parameters.push(param_path);
//         // let get_req = USPGet {
//         //     param_paths: &parameters,
//         //     msg_id: "0",
//         //     max_depth: 5,
//         // };
//         // let msg = get_req.create_msg();
//         // let agent = UspAgent::new(String::from("ops::DarwinTran"));
//         // let record_get = agent.create_record(&msg, &String::from("ops::agent1"));
//     }
// }
