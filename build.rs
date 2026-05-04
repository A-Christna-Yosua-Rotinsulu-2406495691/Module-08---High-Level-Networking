fn main() -> Result<(), Box<dyn std::error::Error>>{
    tonic_build::configure()
        .build_server(true)
        .compile(
            &["proto/services.proto"],  // Path to your proto file
            &["proto"],               // Directory where proto file is located
        )?;
    Ok(())
}