pub mod usp_msg {
    include!(concat!(env!("OUT_DIR"), "/usp.rs"));
    include!(concat!(env!("OUT_DIR"), "/usp_record.rs"));
}
