extern crate protoc_rust;

use protoc_rust::Customize;

use std::path::Path;

fn main() {
    protoc_rust::run(protoc_rust::Args {
        out_dir: Path::new("src").join("ftcodec").to_str().unwrap(),
        includes: &["schema"],
        input: &[
            "schema/snapshot.proto",
            "schema/kline.proto",
            "schema/common.proto",
        ],
        customize: Customize {
            carllerche_bytes_for_bytes: Some(true),
            carllerche_bytes_for_string: Some(true),
            ..Default::default()
        },
    })
    .map_err(|err| panic!(err))
    .unwrap();
}
