// Proka Kernel - A kernel for ProkaOS
// Copyright (C) RainSTR Studio 2025, All Rights Reserved.
//
// The build script, which can link the C and ASM code.


fn main() {
    println!("cargo:rustc-link-arg=-Tlinker.ld");
    println!("cargo:rustc-link-arg=-nostdlib");
    println!("cargo:rustc-link-arg=-no-pie");
}
