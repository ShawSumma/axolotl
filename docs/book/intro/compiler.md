# Compiler

The primary compiler backend will be impmlemented using LLVM, and as such a requirement of the compiler will be the llvm-sys crate, but due to the large nature of this dependancy, an issues that may arise from linking on nonstd targets, this will be an opt in feature, with the interpreter being the fallback, as the interpreter will be written purely in rust, and will be able to operate on abstract syntax trees.

