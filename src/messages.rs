#[derive(RustcDecodable, RustcEncodable)]
pub struct MoveMessage {
    pub from: u8,
    pub to: u8,
}

#[derive(RustcDecodable, RustcEncodable)]
pub struct StateMessage {
    pub my_hand: Vec<u8>,
    pub my_taken: Vec<u8>,
    pub his_hand: Vec<u8>,
    pub his_taken: Vec<u8>,
    pub table: Vec<u8>,
}
