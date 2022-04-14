use std::path::PathBuf;
use std::{env, process};

fn main() {
    let arguments: Vec<String> = env::args().skip(1).collect();
    if arguments.len() < 2 {
        eprintln!("prostc <descriptor_set_path> <outdir>");
        process::exit(1);
    }
    if let [descriptor_set_path, out, ..] = &arguments[..] {
        if let Err(err) = prost_build::Config::new()
            .out_dir(out)
            .file_descriptor_set_path(descriptor_set_path)
            .skip_protoc_run()
            .compile_protos(&[] as &[PathBuf], &[] as &[PathBuf])
        {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}
