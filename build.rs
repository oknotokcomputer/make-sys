use std::process::Command;
use std::{env, vec};
use walkdir::WalkDir;

fn main() {
    let packages_to_install = vec![
        "autoconf",
        "automake",
        "autopoint",
        "build-essential",
        "llvm",
        "clang",
        "libclang-dev",
        "make",
        "cmake",
        "libssl-dev",
        "pkg-config",
        "git",
        "texinfo",
    ];

    // get outdir
    let out_dir = env::var("OUT_DIR").unwrap();

    // apt-get update
    let _ = Command::new("sudo").args(&["apt-get", "update"]).status();

    let args = vec!["apt-get", "install", "-y"];

    let combined_args = args.iter().chain(packages_to_install.iter());

    // install dependencies autoconf and automake
    let _ = Command::new("sudo").args(combined_args).status();

    // Change directory to the out directory
    env::set_current_dir(&out_dir).unwrap();

    // check if directory exists
    if !std::path::Path::new("make").exists() {
        // Clone the repository
        let _ = Command::new("git")
            .args(&["clone", "https://github.com/oknotokcomputer/make"])
            .status();
    }
    // Change directory to the cloned repository
    env::set_current_dir("make").unwrap();

    // Run the necessary commands
    let _ = Command::new("./bootstrap").args(vec!["--pull"]).status();
    let _ = Command::new("./autogen.sh").status();
    let _ = Command::new("./configure").status();
    let _ = Command::new("make").status();

    // Create the super header file
    let headers = vec![        
        "makeint.h",
        "job.h",
        "dep.h",
        "filedef.h",
        "gnumake.h",
        "hash.h",
        "rule.h",
        "shuffle.h",
        "variable.h",
    ];
    let std_headers = vec![
        "sys/types.h",
    ];
    let mut super_header = String::new();
    for header in std_headers {
        super_header.push_str(&format!("#include <{}>\n", header));
    }
    for header in headers {
        let header = header.trim_start_matches("src/");
        if header == "super.h" {
            continue;
        }
        super_header.push_str(&format!("#include \"{}\"\n", header));
    }

    // Write the super header file
    std::fs::write("src/super.h", super_header).unwrap();

    // bindgen the super header file
    bindgen::Builder::default()
        .header("src/super.h")
        .clang_arg("-I./src")
        .clang_arg("-I./lib")
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file("src/bindings.rs")
        .expect("Couldn't write bindings!");

    // Create the archive
    let _ = Command::new("ar")
        .args(&["rcs", "libmake.a"])
        .args(&["src/*.o"])
        .status();

    // print the contents of the out_dir
    for entry in WalkDir::new(&out_dir) {
        let entry = entry.unwrap();
        println!("{}", entry.path().display());
    }

}
