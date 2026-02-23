// Attempt to avoid pain with LLVM, probably.
// LLVM is a nightmare to get set up unless you fancy building
fn main() {
    let output = std::process::Command::new("llvm-config")
        .arg("--prefix")
        .output()
        .expect(r#"
            Failed to run llvm-config. Is llvm-devel installed?

            Try one of the following:

                # Fedora
                - sudo dnf install llvm-devel clang-devel libffi-devel

                # Ubuntu
                - wget https://apt.llvm.org/llvm.sh
                  chmod +x llvm.sh
                  sudo ./llvm.sh 21
                  rm llvm.sh
        "#);

    let prefix = std::str::from_utf8(&output.stdout).unwrap().trim();

    let version_output = std::process::Command::new("llvm-config")
        .arg("--version")
        .output()
        .unwrap();

    let version = std::str::from_utf8(&version_output.stdout).unwrap();
    let major_version = version.split('.').next().unwrap();

    // This turns "21.x.x" into "LLVM_SYS_211_PREFIX"
    // No idea if the dangling 1 actually matters. Probably does, but I lack the brains to
    // work out how to make this work nicely without summoning lucifer
    println!("cargo:rustc-env=LLVM_SYS_{}1_PREFIX={}", major_version, prefix);

    println!("cargo:rustc-link-search=native={}/lib", prefix);
}