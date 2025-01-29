extern crate prost_build;

fn main() {
    prost_build::compile_protos(&["protobuf_file/usp-msg-1-4.proto"], &["protobuf_file/"]).unwrap();
    println!("hahahahah darwin");
}
