use crate::protobuf::usp_msg::Record;
use prost::Message;
pub struct UspMsgHandle {
    buf: Vec<u8>,
}

impl UspMsgHandle {
    fn usp_record_decode(&self) -> Result<Record, prost::DecodeError> {
        Record::decode(&*self.buf)
    }
}
