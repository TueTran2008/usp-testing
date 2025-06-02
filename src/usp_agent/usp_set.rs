use crate::usp_agent::protobuf::usp_msg::set::{UpdateObject, UpdateParamSetting};
// use crate::telemetry::*;

pub struct USPSet {
    allow_partial: bool,
    update_object: Vec<UpdateObject>,
    msg_id: String,
}

pub struct UpdateObjectBuilder {
    obj_path: String,
    param_settings: Vec<UpdateParamSetting>,
}

pub struct UpdateParamSettingBuilder {}

// impl<'a> UspMessageCreate for USPSet<'a> {
//     fn create_msg(&self) -> Msg {
//         let msg_header = Header {
//             msg_id: self.msg_id.into(),
//             msg_type: MsgType::Get.into(),
//         };
//         let set_body = Set {
//             allow_partial: self.allow_partial,
//             update_objs: self.update_object.to_vec(),
//         };
//         let request = Request {
//             req_type: Some(ReqType::Set(set_body)),
//         };
//         let msg_body = Body {
//             msg_body: Some(MsgBody::Request(request)),
//         };
//         Msg {
//             header: Some(msg_header),
//             body: Some(msg_body),
//         }
//     }
// }
