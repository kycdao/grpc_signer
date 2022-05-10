use std::path::PathBuf;

fn main() {
    let out_dir = PathBuf::from("src");
    tonic_build::configure()
        .out_dir(out_dir)
        //.file_descriptor_set_path(out_dir.join("signer_descriptor.bin"))
        .compile(&["proto/signer.proto"], &["proto"])
        .unwrap();
}
