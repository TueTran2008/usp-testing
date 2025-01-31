use crate::protobuf::usp_msg::Record;

pub struct UspAgent {
    eid: String, // Endpoint ID of agent
                 //tx_record: Record,
                 //rx_record: Record,
}

impl UspAgent {
    pub fn new(eid: String) -> Self {
        UspAgent { eid }
    }
    fn get_eid(&self) -> &str {
        &self.eid
    }
    pub fn validate_eid(&self, target: &str) -> Result<(), ()> {
        if let eid = self.eid.as_str() {
            if eid == target {
                Ok(())
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}
