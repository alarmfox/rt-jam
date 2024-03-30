use crate::protos::packet_wrapper::packet_wrapper::PacketType;

impl std::fmt::Display for PacketType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PacketType::RSA_PUB_KEY => f.write_str("RSA_PUB_KEY"),
            PacketType::AES_KEY => f.write_str("AES_KEY"), 
            PacketType::MEDIA => f.write_str("MEDIA"),
            PacketType::CONNECTION => f.write_str("CONNECTION"),
        }
    }
}
