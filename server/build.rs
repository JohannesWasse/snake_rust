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

    /*
       tonic_build::configure()
           .server_mod_attribute("attrs", "#[cfg(feature = \"server\")]")
           .server_attribute("Echo", "#[derive(PartialEq)]")
           .client_mod_attribute("attrs", "#[cfg(feature = \"client\")]")
           .client_attribute("Echo", "#[derive(PartialEq)]")
           .compile(&["proto/attrs/attrs.proto"], &["proto"])
           .unwrap();

       tonic_build::configure()
           .build_server(false)
           .compile(
               &["proto/googleapis/google/pubsub/v1/pubsub.proto"],
               &["proto/googleapis"],
           )
           .unwrap();

    */
}
