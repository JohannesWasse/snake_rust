use std::path::{Path, PathBuf};
fn main() {
    tonic_build::configure()
        .build_server(true)
        .out_dir(Path::new("src/proto"))
        .compile(
            &[PathBuf::from("../proto/snake.proto")
                .canonicalize()
                .unwrap()],
            &[PathBuf::from("../proto").canonicalize().unwrap()],
        )
        .unwrap();
}
