fn main() -> Result<(), Box<dyn std::error::Error>>{
    tonic_build::configure()
        .build_server(true)
        .compile(
            &["proto/services_proto"],  // Path to yout proto file
            &["proto"],               // Directory where proto file is located
        )?;
    Ok(())
}