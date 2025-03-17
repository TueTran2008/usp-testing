use crate::protobuf::usp_msg::body::MsgBody;
use crate::protobuf::usp_msg::header::MsgType;

pub mod usp_msg {
    include!(concat!(env!("OUT_DIR"), "/usp.rs"));
    include!(concat!(env!("OUT_DIR"), "/usp_record.rs"));
}

pub struct USPGet;

impl USPGet {
    pub fn create_get(&self) {
        MsgBody::encode
    }
}
