fn main() -> Result<(), Box<dyn std::error::Error>> 
{
    tonic_build::compile_protos("libs/fc_server/src/services/grpc/proto/proto/packets.proto")?;
    tonic_build::compile_protos("libs/fc_server/src/services/grpc/proto/proto/task.proto")?;
    Ok(())
}