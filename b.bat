@echo off
setlocal

set clang=C:\dnload\general-utils\clang+llvm-18.1.8-x86_64-pc-windows-msvc
set gcc_slash=C:/dnload/general-utils/arm-gnu-toolchain-14.2.rel1-mingw-w64-i686-arm-none-eabi

set BINDGEN_EXTRA_CLANG_ARGS=--sysroot=%gcc_slash%/arm-none-eabi
set PATH=%clang%\bin;%gcc_slash%/bin;%PATH%

cargo build %*
endlocal

