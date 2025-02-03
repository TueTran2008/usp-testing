pub struct UspAgent {
    eid: String,
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
