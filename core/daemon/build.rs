pub fn main() -> std::io::Result<()> {
    tonic_build::compile_protos("../../contracts/db-cluster-daemon.proto")?;
    Ok(())
}
