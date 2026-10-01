fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Use local protoc if PROTOC env is set, otherwise system protoc
    tonic_build::compile_protos("../proto/hacp/control/v1/control_plane.proto")?;
    Ok(())
}
