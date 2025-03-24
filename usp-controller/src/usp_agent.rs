use crate::protobuf::usp_msg::header::MsgType;
use crate::protobuf::usp_msg::record::{PayloadSecurity, RecordType};
use crate::protobuf::usp_msg::{self, Msg, NoSessionContextRecord};
use crate::protobuf::usp_msg::{body::MsgBody, Record};
use crate::usp_msg_handle::MessageHandler;
use prost::Message;
use tracing::{error, info};
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum UspError {
    // General success
    Ok = 0,

    // Message error codes
    GeneralFailure = 7000,
    MessageNotUnderstood = 7001,
    RequestDenied = 7002,
    InternalError = 7003,
    InvalidArguments = 7004,
    ResourcesExceeded = 7005,
    PermissionDenied = 7006,
    InvalidConfiguration = 7007,

    // ParamError codes
    InvalidPathSyntax = 7008,
    ParamActionFailed = 7009,
    UnsupportedParam = 7010,
    InvalidType = 7011,
    InvalidValue = 7012,
    ParamReadOnly = 7013,
    ValueConflict = 7014,

    // CRUD Operation Errors
    CrudFailure = 7015,
    ObjectDoesNotExist = 7016,
    CreationFailure = 7017,
    NotATable = 7018,
    ObjectNotCreatable = 7019,
    SetFailure = 7020,
    RequiredParamFailed = 7021,

    // Command Errors
    CommandFailure = 7022,
    CommandCancelled = 7023,
    ObjectNotDeletable = 7024,
    UniqueKeyConflict = 7025,
    InvalidPath = 7026,
    InvalidCommandArgs = 7027,

    // Brokered USP Record Errors
    RecordNotParsed = 7100,
    SecureSessionRequired = 7101,
    SecureSessionNotSupported = 7102,
    SegmentationNotSupported = 7103,
    RecordFieldInvalid = 7104,

    // Vendor-defined errors (range 7800-7999)
    VendorDefined(i32),
}

impl UspError {
    /// Convert an `i32` into an `Option<UspError>`.
    pub fn from_code(code: i32) -> Option<Self> {
        match code {
            0 => Some(Self::Ok),
            7000 => Some(Self::GeneralFailure),
            7001 => Some(Self::MessageNotUnderstood),
            7002 => Some(Self::RequestDenied),
            7003 => Some(Self::InternalError),
            7004 => Some(Self::InvalidArguments),
            7005 => Some(Self::ResourcesExceeded),
            7006 => Some(Self::PermissionDenied),
            7007 => Some(Self::InvalidConfiguration),
            7008 => Some(Self::InvalidPathSyntax),
            7009 => Some(Self::ParamActionFailed),
            7010 => Some(Self::UnsupportedParam),
            7011 => Some(Self::InvalidType),
            7012 => Some(Self::InvalidValue),
            7013 => Some(Self::ParamReadOnly),
            7014 => Some(Self::ValueConflict),
            7015 => Some(Self::CrudFailure),
            7016 => Some(Self::ObjectDoesNotExist),
            7017 => Some(Self::CreationFailure),
            7018 => Some(Self::NotATable),
            7019 => Some(Self::ObjectNotCreatable),
            7020 => Some(Self::SetFailure),
            7021 => Some(Self::RequiredParamFailed),
            7022 => Some(Self::CommandFailure),
            7023 => Some(Self::CommandCancelled),
            7024 => Some(Self::ObjectNotDeletable),
            7025 => Some(Self::UniqueKeyConflict),
            7026 => Some(Self::InvalidPath),
            7027 => Some(Self::InvalidCommandArgs),
            7100 => Some(Self::RecordNotParsed),
            7101 => Some(Self::SecureSessionRequired),
            7102 => Some(Self::SecureSessionNotSupported),
            7103 => Some(Self::SegmentationNotSupported),
            7104 => Some(Self::RecordFieldInvalid),
            7800..=7999 => Some(Self::VendorDefined(code)),
            _ => None,
        }
    }

    // Convert the `UspError` into an `i32` code.
    // pub fn as_code(&self) -> i32 {
    //     *self as i32
    // }
}

pub struct UspAgent {
    eid: String,
}

struct GetResponseHandle;

impl MessageHandler for GetResponseHandle {
    fn message_type(&self) -> MsgType {
        MsgType::Get
    }

    fn handle(&self, msg: &Msg, from_eid: &str) {
        let body = msg.body.as_ref().unwrap();
        let msg_body = body.msg_body.as_ref().unwrap();
        match msg_body {
            MsgBody::Response(res) => {
                // do something
                info!("Message get response {:?}", res);
            }
            _ => {
                error!("Not and get response");
            }
        }
    }
}

impl UspAgent {
    pub fn new(eid: String) -> Self {
        UspAgent { eid }
    }
    fn get_eid(&self) -> &str {
        &self.eid
    }
    pub fn validate_eid(&self, target: &str) -> Result<(), ()> {
        let eid = self.eid.as_str();
        if eid == target {
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn create_record(&self, msg: &Msg, to_eid: &String) -> Record {
        let usp_message = msg.encode_to_vec();
        Record {
            version: "1.3".to_string(),
            to_id: to_eid.to_string(),
            from_id: self.get_eid().to_string(),
            payload_security: PayloadSecurity::Plaintext.into(),
            record_type: Some(RecordType::NoSessionContext(NoSessionContextRecord {
                payload: usp_message,
            })),
            mac_signature: Vec::new(),
            sender_cert: Vec::new(),
        }
    }
}
