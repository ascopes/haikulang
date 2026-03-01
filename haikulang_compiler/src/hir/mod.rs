//! Higher intermediate representation for Haikulang programs.
//!
//! This acts as an intermediate format that is produced by flattening
//! the AST into instructions and symbols such that it can be mapped
//! almost 1-to-1 to LLVM instructions later on.
mod arena;
pub mod context;
pub mod lowerer;
pub mod nodes;
mod sym;
