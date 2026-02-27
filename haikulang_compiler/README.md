# haikulang_llvm

The LLVM backend for the Haikulang compiler.

This is a little tricky as you have to have the right version of LLVM installed for this to actually build. The
required version is defined as a feature on the Inkwell dependency in the Cargo.toml of this project. It expects
you have `llvm-config` installed and on your path pointing to the right version of LLVM.

As of right now, we only dynamic link as dependencies become a huge pain.