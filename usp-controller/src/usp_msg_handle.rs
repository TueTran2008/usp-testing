use crate::protobuf::usp_msg::record::RecordType;
use crate::protobuf::usp_msg::Msg;
use crate::protobuf::usp_msg::Record;
use log::{error, info};
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
    pub fn usp_record_unpack(record: Record) -> Result<Msg, ()> {
        if let Some(record_type) = record.record_type {
            match record_type {
                RecordType::NoSessionContext(record_type) => {
                    if record_type.payload.is_empty() == true {
                        error!("USP No Session Context is Empty");
                        Err(())
                    } else {
                        match Msg::decode(record_type.payload.as_slice()) {
                            Ok(msg) => Ok(msg),
                            Err(_) => Err(()),
                        }
                    }
                }
                _ => {
                    error!("USP Record contained no USP message (or message was in a E2E session context). Ignoring USP Record");
                    Err(())
                }
            }
        } else {
            Err(())
        }
    }
}
