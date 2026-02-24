# haikulang_llvm

The LLVM backend for the Haikulang compiler.

This is a little tricky as you have to have the right version of LLVM installed for this to actually build. The
required version is defined as a feature on the Inkwell dependency in the Cargo.toml of this project. It expects
you have `llvm-config` installed and on your path pointing to the right version of LLVM.

As of right now, we force static-link LLVM into the binary as it makes life a bit easier for us during development.
If we ever distribute anything, this should probably be dynamic unless we review the full license of LLVM carefully
first.