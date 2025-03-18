// use crate::protobuf::usp_msg::body::MsgBody;
// use crate::protobuf::usp_msg::header::MsgType;
// use crate::protobuf::usp_msg::record::{PayloadSecurity, RecordType};
// use crate::protobuf::usp_msg::request::ReqType;
// use crate::protobuf::usp_msg::Record;

pub mod usp_msg {
    include!(concat!(env!("OUT_DIR"), "/usp.rs"));
    include!(concat!(env!("OUT_DIR"), "/usp_record.rs"));
}
use crate::usp_agent::{UspAgent, UspError};
use prost::Message;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;
use tracing::{error, info, warn};
use usp_msg::{header::MsgType, Msg, Record};


pub struct UspMsgHandle {
    buf: Vec<u8>,
    agent: UspAgent,
}

pub struct MessageDispatcher {
    handlers: HashMap<MsgType, Arc<dyn MessageHandler>>,
}

// Builder for MessageDispatcher
pub struct MessageDispatcherBuilder {
    handlers: HashMap<MsgType, Arc<dyn MessageHandler>>,
}
#[derive(Error, Debug)]
pub enum MessageDispatcherError {
    #[error("Handler already exists for message type: {0}")]
    HandlerAlreadyExist(String),
    #[error("No handler found for message type: {0}")]
    NoHandlerFound(String),
}

pub trait UspMessageCreate {
    fn create_msg(&self) -> usp_msg::Msg;
}
// Trait for message handlers with dynamic dispatch
// Trait for handling different message types
pub trait MessageHandler: Send + Sync {
    fn handle(&self, msg: &, from_eid: &str);
    fn message_type(&self) -> MsgType;
}

impl MessageDispatcherBuilder {
    pub fn new() -> Self {
        let handlers = HashMap::new();
        Self { handlers }
    }

    fn add_handler<H: MessageHandler + 'static>(
        mut self,
        msg_handle: H,
    ) -> Result<Self, MessageDispatcherError> {
        let msg_type = msg_handle.message_type();
        if self.handlers.contains_key(&msg_type) {
            return Err(MessageDispatcherError::HandlerAlreadyExist(
                msg_type.as_str_name().to_string(),
            ));
        }
        self.handlers.insert(msg_type, Arc::new(msg_handle));
        Ok(self)
    }
    pub fn build(self) -> MessageDispatcher {
        MessageDispatcher {
            handlers: self.handlers,
        }
    }
}

impl MessageDispatcher {
    pub fn register_handle(
        &mut self,
        handler: Arc<dyn MessageHandler>,
    ) -> Result<(), MessageDispatcherError> {
        let msg_type = handler.message_type();
        if self.handlers.contains_key(&msg_type) {
            return Err(MessageDispatcherError::HandlerAlreadyExist(
                msg_type.as_str_name().to_string(),
            ));
        }
        self.handlers.insert(msg_type, handler);
        Ok(())
    }
    // This is controller => I prefer we will do the message response handle first
    pub fn message_handle(
        &self,
        msg: &Msg,
        from_eid: &String,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let header = msg.header.as_ref().ok_or_else(|| {
            error!("Receive message with invalid header");
            Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "No header",
            ))
        })?;

        let msg_handle = self.handlers.get(&header.msg_type()).ok_or_else(|| {
            let ref msg_type = header.msg_type();
            error!(message_type = %msg_type.as_str_name(), "No handler for Message Header");
            Box::new(MessageDispatcherError::NoHandlerFound(
                msg_type.as_str_name().to_string(),
            ))
        })?;
        msg_handle.handle(msg, from_eid);
        Ok(())
    }
}

impl UspMsgHandle {
    pub fn usp_record_decode(input: &[u8]) -> Result<Record, prost::DecodeError> {
        Record::decode(input)
    }
    pub fn usp_record_debug(record: &Record) {
        info!("Receive record");
        info!("Version {}", record.version);
        info!("To ID {}", record.to_id);
        info!("From ID {}", record.from_id);
    }
    // pub fn usp_msg_debug(msg: &Msg) {
    //     info!("Message Header");
    //     info!("------------START of Message Header-------------");
    //     if let Some(ref header) = msg.header {
    //         info!("ID: {}", header.msg_id);
    //         info!("MsgType: {}", header.msg_type().as_str_name());
    //     }
    //     info!("------------END of Message Header-------------");
    //     info!("**********************************************");
    //     info!("------------Start of Message BODY-------------");
    //     if let Some(ref body) = msg.body {
    //         if let Some(ref msg_body) = body.msg_body {
    //             match msg_body {
    //                 MsgBody::Error(err) => {
    //                     let mut i = 0;
    //                     info!("Error code {}", err.err_code);
    //                     info!("Error message {}", err.err_msg);
    //                     for err_param in &err.param_errs {
    //                         info!("Error parameter {} path {}", i, err_param.param_path);
    //                         info!("Error parameter {} code {}", i, err_param.err_code);
    //                         info!("Error parameter {} messgage {}", i, err_param.err_msg);
    //                         i = i + 1;
    //                     }
    //                 }
    //                 MsgBody::Request(request) => {
    //                     if let Some(ref req_type) = request.req_type {
    //                         match req_type {
    //                             // ReqType::Get(get_request) => {}
    //
    //                             _ => panic!("Hehe"),
    //                         }
    //                     }
    //                 }
    //                 _ => {
    //                     info!("do things");
    //                 }
    //             }
    //         }
    //     }
    // }
    pub fn usp_record_unpack(record: &Record) -> Result<Msg, ()> {
        if let Some(ref record_type) = record.record_type {
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

    pub fn usp_validate_record(record: &Record, agent: UspAgent) -> Result<(), UspError> {
        match agent.validate_eid(record.to_id.as_str()) {
            Ok(_) => {}
            Err(_) => {
                return Err(UspError::RequestDenied);
            }
        }
        if record.from_id.is_empty() == true {
            warn!("Ignoring USP record as it was addressed to endpoint_id=%s");
            return Err(UspError::RecordFieldInvalid);
        }
        if record.payload_security != PayloadSecurity::Plaintext as i32 {
            warn!("Not performing integrity check on non-payload fields of received USP Record");
            return Err(UspError::SecureSessionNotSupported);
        }
        if record.mac_signature.is_empty() == false {
            warn!("WARNING: Not performing integrity check on non-payload fields of received USP Record");
        }
        if record.sender_cert.is_empty() == false {
            warn!("Skipping sender certificate verification");
        }
        if let Some(ref record_type) = record.record_type {
            match record_type {
                RecordType::NoSessionContext(_) => {}
                _ => {
                    warn!("Ignoring USP record as it does not contain a payload");
                    return Err(UspError::SecureSessionNotSupported);
                }
            }
        }
        Ok(())
    }
}
