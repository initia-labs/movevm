use initia_move_compiler::{execute, Command};
use move_cli::{base::build::Build, Move};
use std::{env::current_dir, fs, io, path::PathBuf, str::FromStr};

fn main() {
    let root = current_dir().unwrap();
    let bin_dir = root.join("precompile/binaries");
    let stdlib_bin_dir = bin_dir.join("stdlib");
    let minlib_bin_dir = bin_dir.join("minlib");
    let tests_bin_dir = bin_dir.join("tests");
    let hooks_bin_dir = bin_dir.join("hooks");

    let modules = vec![
        [
            "move_stdlib",
            "MoveStdlib",
            stdlib_bin_dir.to_str().unwrap(),
        ],
        [
            "move_nursery",
            "MoveNursery",
            stdlib_bin_dir.to_str().unwrap(),
        ],
        [
            "initia_stdlib",
            "InitiaStdlib",
            stdlib_bin_dir.to_str().unwrap(),
        ],
        [
            "move_stdlib",
            "MoveStdlib",
            minlib_bin_dir.to_str().unwrap(),
        ],
        [
            "move_nursery",
            "MoveNursery",
            minlib_bin_dir.to_str().unwrap(),
        ],
        [
            "minitia_stdlib",
            "MinitiaStdlib",
            minlib_bin_dir.to_str().unwrap(),
        ],
        [
            "initia_hooks",
            "InitiaHooks",
            hooks_bin_dir.to_str().unwrap(),
        ],
        ["tests", "tests", tests_bin_dir.to_str().unwrap()],
    ];

    fs::remove_dir_all(bin_dir).expect("failed to clear binaries dir");
    fs::create_dir_all(stdlib_bin_dir.clone()).expect("failed to create binaries/stdlib dir");
    fs::create_dir_all(minlib_bin_dir.clone()).expect("failed to create binaries/minlib dir");
    fs::create_dir_all(tests_bin_dir.clone()).expect("failed to create binaries/tests dir");
    fs::create_dir_all(hooks_bin_dir.clone()).expect("failed to create binaries/hooks dir");

    for [p, m, d] in modules {
        // compile modules & scripts
        build(root.join(PathBuf::from(format!("precompile/modules/{p}", p = p))));

        // copy modules
        copy(
            root.join(PathBuf::from(format!(
                "precompile/modules/{p}/build/{m}/bytecode_modules",
                p = p,
                m = m
            ))),
            PathBuf::from_str(d).unwrap(),
        )
        .expect("copy failed");

        // copy scripts; ignore error
        let _ = copy(
            root.join(PathBuf::from(format!(
                "precompile/modules/{p}/build/{m}/bytecode_scripts",
                p = p,
                m = m
            ))),
            PathBuf::from_str(d).unwrap(),
        );
    }
}

fn build(p: PathBuf) {
    let package_path = Some(p);

    let build_config = move_package::BuildConfig {
        install_dir: package_path.clone(),
        ..Default::default()
    };

    let arg = Move {
        package_path,
        verbose: true,
        build_config,
    };

    execute(arg, Command::Build(Build)).expect("error occurred while compiling contracts");
}

fn copy(source: PathBuf, dest: PathBuf) -> io::Result<()> {
    for entry in fs::read_dir(source)? {
        let entry = entry?;
        let filetype = entry.file_type()?;
        if filetype.is_dir() {
            continue;
        }

        let extension = &entry.path().extension().unwrap().to_os_string();
        if extension == "mv" {
            fs::copy(entry.path(), dest.join(entry.file_name()))?;
        }
    }

    Ok(())
}
