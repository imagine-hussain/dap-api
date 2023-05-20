//! https://microsoft.github.io/debug-adapter-protocol/specification

pub mod requests;
pub mod request_args;


pub trait ProtocolMessage {
    // to_json
    // get_sequence_number
    fn get_type(&self) -> ProtocolMessageType;
}

pub enum ProtocolMessageType {
    Request,
    Response,
    Event,
    Other(String)
}

pub enum Number {
    Int(i64),
    Float(f64),
}

// TODO:
type Source = ();
