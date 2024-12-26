use std::{
    env, fs,
    process::{Command, ExitStatus},
};

use clap::Args;

use crate::utils::cargo;

#[derive(Debug, Args, Clone)]
pub struct BenchArg {
    /// Package Prototyper and Test-Kernel
    #[clap(long)]
    pub pack: bool,
}

#[must_use]
pub fn run(arg: &BenchArg) -> Option<ExitStatus> {
    let arch = "riscv64imac-unknown-none-elf";
    let current_dir = env::current_dir();
    let target_dir = current_dir
        .as_ref()
        .unwrap()
        .join("target")
        .join(arch)
        .join("release");

    info!("Building bench kernel");
    cargo::Cargo::new("build")
        .package("rustsbi-bench-kernel")
        .target(arch)
        .release()
        .status()
        .ok()?;

    info!("Copy to binary");
    let exit_status = Command::new("rust-objcopy")
        .args(["-O", "binary"])
        .arg("--binary-architecture=riscv64")
        .arg(target_dir.join("rustsbi-bench-kernel"))
        .arg(target_dir.join("rustsbi-bench-kernel.bin"))
        .status()
        .ok()?;

    if arg.pack {
        info!("Pack to image");
        match fs::exists(target_dir.join("rustsbi-prototyper.bin")) {
            Ok(true) => {}
            Ok(false) => {
                panic!(" Couldn't open \"rustsbi-prototyper.bin\": No such file or directory. Please compile Prototyper first");
            }
            Err(_) => {
                panic!("Can't check existence of file rustsbi-prototyper.bin, please compile Prototyper first");
            }
        }
        fs::copy(
            current_dir
                .as_ref()
                .unwrap()
                .join("bench-kernel")
                .join("scripts")
                .join("rustsbi-bench-kernel.its"),
            target_dir.join("rustsbi-bench-kernel.its"),
        )
        .ok()?;
        env::set_current_dir(&target_dir).ok()?;
        Command::new("mkimage")
            .args(["-f", "rustsbi-bench-kernel.its"])
            .arg("rustsbi-bench-kernel.itb")
            .status()
            .ok()?;
        fs::remove_file(env::current_dir().unwrap().join("rustsbi-bench-kernel.its")).ok()?;
    }
    Some(exit_status)
}
