extern crate protoc_grpcio;

fn main() {
    let proto_root = "src/protos";
    println!("cargo:rerun-if-changed={}", proto_root);
    protoc_grpcio::compile_grpc_protos(
        &["defs/rustserver.proto"],
        &[proto_root],
        &proto_root
    ).expect("Failed to compile gRPC definitions!");
}