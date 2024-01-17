use crate::ast::AST;

pub mod asm;

pub use asm::AssemblyCodeGenerator;

pub trait Codegen {
    fn codegen(ast: AST, optimized: bool) -> String;
}

pub fn codegen<T: Codegen>(ast: AST, optimized: bool) -> String {
    T::codegen(ast, optimized)
}
