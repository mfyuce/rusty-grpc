fn main () -> Result<(), Box<dyn std::error::Error>> {
    tonic_build::compile_protos("../protos/voting.proto")?;
    // tonic_build::compile_protos("../protos/gnmi_ext.proto")?;
    tonic_build::compile_protos("../protos/gnmi.proto")?;
    Ok(())
}
