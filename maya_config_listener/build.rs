fn main () -> Result<(), Box<dyn std::error::Error>> {
    // // tonic_build::configure().out_dir("./src").build_server(false);
    // tonic_build::compile_protos("../protos/voting.proto")?;
    // tonic_build::compile_protos("../protos/gnmi_ext.proto")?;
    // tonic_build::compile_protos("../protos/gnmi.proto")?;

    // 
    // 
    // let proto_file = "/home/fatihyuce/work/projects/maya3/ng-sdn/tonic/rusty-grpc/protos/voting.proto";
    // tonic_build::configure()
    //     .build_server(true)
    //     .out_dir("./src")
    //     .compile(&[proto_file], &["."])
    //     .unwrap_or_else(|e| panic!("protobuf compile error: {}", e));

    // tonic_build::configure()
    //     .build_server(false).compile(
    //         &["../protos/gnmi_ext.proto"],
    //         &["proto/gnmi_ext"],
    //     )?;
    // tonic_build::configure()
    //     .build_server(false).compile(
    //         &["../protos/gnmi.proto"],
    //         &["proto/gnmi"],
    //     )?;
    Ok(())
}
