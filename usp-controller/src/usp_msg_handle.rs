use crate::protobuf::usp_msg::Record;
use log::info;
use prost::Message;
pub struct UspMsgHandle {
    buf: Vec<u8>,
}

impl UspMsgHandle {
    pub fn usp_record_decode(input: &[u8]) -> Result<Record, prost::DecodeError> {
        Record::decode(input)
    }
    pub fn usp_record_debug(record: Record) {
        info!("Receive record");
        info!("Version {}", record.version);
        info!("To ID {}", record.to_id);
        info!("From ID {}", record.from_id);
    }
}
