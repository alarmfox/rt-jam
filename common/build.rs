fn main() {
    // Use this in build.rs
    protobuf_codegen::Codegen::new()
        // All inputs and imports from the inputs must reside in `includes` directories.
        .includes(&["src/protos"])
        // Inputs must reside in some of include paths.
        .input("src/protos/aes_packet.proto")
        .input("src/protos/connection_packet.proto")
        .input("src/protos/media_packet.proto")
        .input("src/protos/packet_wrapper.proto")
        .input("src/protos/rsa_packet.proto")
        // Specify output directory relative to Cargo output directory.
        .cargo_out_dir("protos")
        .run_from_script();
}
