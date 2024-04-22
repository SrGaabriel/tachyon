use rsa::traits::PublicKeyParts;
use crate::define_packet;
use crate::network::connection::PlayerConnection;
use crate::packet::PacketDefinition;
use crate::server::TachyonServer;
use crate::packet::types::PacketStructure;
use crate::security::transform_byte_vec;

define_packet!(0x01, ClientboundEncryptionRequestPacket {
    server_id: String,
    public_key: Vec<u8>,
    verify_token: Vec<u8>
});

define_packet!(0x01, ServerboundEncryptionResponsePacket {
    shared_secret: Vec<u8>,
    verify_token: Vec<u8>
});

pub fn enable_encryption(server: &mut TachyonServer, connection: &mut PlayerConnection) {
    let mut verify_token = crate::security::generate_verify_token();
    transform_byte_vec(&mut verify_token);
    let server_private_key = &server.keypair.private;
    let mut encoded_public_key = rsa_der::public_key_to_der(&server_private_key.n().to_bytes_be(), &server_private_key.e().to_bytes_be());
    transform_byte_vec(&mut encoded_public_key);

    connection.security_info = Some(crate::network::connection::SecurityInfo {
        verify_token: verify_token.clone(),
        public_key: server.keypair.public.clone()
    });

    let mut encryption_packet = ClientboundEncryptionRequestPacket {
        server_id: String::from(""),
        public_key: encoded_public_key,
        verify_token
    };
    println!("Packet debug info: {:?}", encryption_packet);
    println!("Public key length: {}", encryption_packet.public_key.len());
    encryption_packet.write_data(&mut connection.stream);
}