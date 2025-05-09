based partially on:
<https://github.com/embassy-rs/trouble/tree/main/examples/apache-nimble>

`memory.x` and `build.rs` and `.cargo/config.toml` based on:
<https://github.com/NorbertSzydlik/rust-arduino-nano-33-ble/blob/50c97b32cc5e115ca8ef50ab08eba4f05170cad7/memory.x>
and:
<https://github.com/arduino/ArduinoCore-mbed/blob/376aabf3487c85c4f4fc1cfbf2eee922b0c10085/variants/ARDUINO_NANO33BLE/linker_script.ld#L1-L8>
and:
<https://github.com/embassy-rs/embassy/blob/ca5ebe859a40af38a889553334afbcc22cf1aba7/examples/nrf52840/build.rs>
and:
<https://github.com/NorbertSzydlik/rust-arduino-nano-33-ble/blob/50c97b32cc5e115ca8ef50ab08eba4f05170cad7/.cargo/config.toml>
and:
<https://github.com/embassy-rs/embassy/blob/ca5ebe859a40af38a889553334afbcc22cf1aba7/examples/nrf52840/.cargo/config.toml>

## NOTE

### bindgen

Seems to need [bindgen requirements](https://rust-lang.github.io/rust-bindgen/requirements.html)
to be installed, notably LLVM
(possibly prebuilt e.g. on Windows)
and the env variable `LIBCLANG_PATH`
set appropriately to the location of `libclang.dll`.
(Unfortunately, this seems to be a huge 1GB pack
at the time of writing.)


Otherwise, I get the following error:

       Compiling apache-nimble v0.1.0 (https://github.com/benbrittain/apache-nimble-sys.git?branch=master#8f8f66a3)
    error: failed to run custom build command for `apache-nimble-sys v0.0.1 (https://github.com/benbrittain/apache-nimble-sys.git?branch=master#8f8f66a3)`
    
    Caused by:
      process didn't exit successfully: `C:\prog\blergh1\target\debug\build\apache-nimble-sys-c44390088eb918cf\build-script-build` (exit code: 101)
      --- stdout
      cargo:rerun-if-changed=include
      cargo:rerun-if-env-changed=TARGET
      cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS_thumbv7em-none-eabi
      cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS_thumbv7em_none_eabi
      cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS
    
      --- stderr
    
      thread 'main' panicked at C:\Users\Mateusz\.cargo\registry\src\index.crates.io-1949cf8c6b5b557f\bindgen-0.69.5\lib.rs:622:31:
      Unable to find libclang: "couldn't find any valid shared libraries matching: ['clang.dll', 'libclang.dll'], set the `LIBCLANG_PATH` environment variable to a path where one of these files can be found (invalid: [])"
      note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
    warning: build failed, waiting for other jobs to finish...

### cross-compiler

The cross-compiler also needs to be relatively modern.
I first tried the compiler that is part of the
Arduino Core for nano-33-ble (a.k.a. "Arduino Core for Mbed ..."
or something like this),
but I got the following errors:

       Compiling apache-nimble v0.1.0 (https://github.com/benbrittain/apache-nimble-sys.git?branch=master#8f8f66a3)
    warning: apache-nimble@0.1.0: ../mynewt-nimble/nimble/controller/src\ble_ll_hci_supp_cmd.c:295:5: error: initializer element is not constant
    warning: apache-nimble@0.1.0:      octet_0,
    warning: apache-nimble@0.1.0:      ^~~~~~~
    warning: apache-nimble@0.1.0: ../mynewt-nimble/nimble/controller/src\ble_ll_hci_supp_cmd.c:295:5: note: (near initialization for 'g_ble_ll_hci_supp_cmds[0]')
    warning: apache-nimble@0.1.0: ../mynewt-nimble/nimble/controller/src\ble_ll_hci_supp_cmd.c:297:5: error: initializer element is not constant
    warning: apache-nimble@0.1.0:      octet_2,
    warning: apache-nimble@0.1.0:      ^~~~~~~
    warning: apache-nimble@0.1.0: ../mynewt-nimble/nimble/controller/src\ble_ll_hci_supp_cmd.c:297:5: note: (near initialization for 'g_ble_ll_hci_supp_cmds[2]')
    warning: apache-nimble@0.1.0: ../mynewt-nimble/nimble/controller/src\ble_ll_hci_supp_cmd.c:300:5: error: initializer element is not constant
    (...)
    error: failed to run custom build command for `apache-nimble v0.1.0 (https://github.com/benbrittain/apache-nimble-sys.git?branch=master#8f8f66a3)`
    
    Caused by:
      process didn't exit successfully: `C:\prog\blergh1\target\debug\build\apache-nimble-04252a9f6c7bd595\build-script-build` (exit code: 1)
      --- stdout
      cargo:rerun-if-changed=nrfx_include
      OPT_LEVEL = Some(0)
      OUT_DIR = Some(C:\prog\blergh1\target\thumbv7em-none-eabi\debug\build\apache-nimble-1b316b1bda9c7042\out)
      TARGET = Some(thumbv7em-none-eabi)
      HOST = Some(x86_64-pc-windows-msvc)
      cargo:rerun-if-env-changed=CC_thumbv7em-none-eabi
      CC_thumbv7em-none-eabi = None
      cargo:rerun-if-env-changed=CC_thumbv7em_none_eabi
      CC_thumbv7em_none_eabi = None
      cargo:rerun-if-env-changed=TARGET_CC
      TARGET_CC = None
      cargo:rerun-if-env-changed=CC
      CC = None
      cargo:rerun-if-env-changed=CROSS_COMPILE
      CROSS_COMPILE = None
      RUSTC_LINKER = None
      cargo:rerun-if-env-changed=CC_ENABLE_DEBUG_OUTPUT
      RUSTC_WRAPPER = None
      cargo:rerun-if-env-changed=CRATE_CC_NO_DEFAULTS
      CRATE_CC_NO_DEFAULTS = None
      DEBUG = Some(true)
      CARGO_CFG_TARGET_FEATURE = None
      cargo:rerun-if-env-changed=CFLAGS
      CFLAGS = None
      cargo:rerun-if-env-changed=TARGET_CFLAGS
      TARGET_CFLAGS = None
      cargo:rerun-if-env-changed=CFLAGS_thumbv7em_none_eabi
      CFLAGS_thumbv7em_none_eabi = None
      cargo:rerun-if-env-changed=CFLAGS_thumbv7em-none-eabi
      CFLAGS_thumbv7em-none-eabi = None
      CARGO_ENCODED_RUSTFLAGS = Some()
      cargo:warning=../mynewt-nimble/nimble/controller/src\ble_ll_hci_supp_cmd.c:295:5: error: initializer element is not constant
      cargo:warning=     octet_0,
      cargo:warning=     ^~~~~~~
      cargo:warning=../mynewt-nimble/nimble/controller/src\ble_ll_hci_supp_cmd.c:295:5: note: (near initialization for 'g_ble_ll_hci_supp_cmds[0]')
      cargo:warning=../mynewt-nimble/nimble/controller/src\ble_ll_hci_supp_cmd.c:297:5: error: initializer element is not constant
      cargo:warning=     octet_2,
      cargo:warning=     ^~~~~~~
      cargo:warning=../mynewt-nimble/nimble/controller/src\ble_ll_hci_supp_cmd.c:297:5: note: (near initialization for 'g_ble_ll_hci_supp_cmds[2]')
      cargo:warning=../mynewt-nimble/nimble/controller/src\ble_ll_hci_supp_cmd.c:300:5: error: initializer element is not constant
    (...)
      --- stderr
    
    
      error occurred in cc-rs: command did not execute successfully (status code exit code: 1): "arm-none-eabi-gcc" "-O0" "-ffunction-sections" "-fdata-sections" "-g" "-fno-omit-frame-pointer" "-mthumb" "-march=armv7e-m" "-I" "../mynewt-nimble/nimble/transport/include" "-I" "../mynewt-nimble/nimble/include" "-I" "C:\\prog\\blergh1\\target\\thumbv7em-none-eabi\\debug\\build\\apache-nimble-1b316b1bda9c7042\\out" "-I" "../apache-nimble-sys/include" "-I" "../mynewt-nimble/porting/nimble/include" "-I" "../mynewt-nimble/ext/tinycrypt/include" "-I" "../mynewt-nimble/nimble/controller/include" "-I" "../mynewt-nimble/nimble/drivers/nrf5x/include" "-I" "../CMSIS_5/CMSIS/Core/Include" "-I" "../nrfx" "-I" "../nrfx/mdk" "-I" "../nrfx/hal" "-I" "nrfx_include" "-DDEFINE_EMBASSY" "-DNIMBLE_CFG_CONTROLLER=1" "-DNRF52840_XXAA" "-o" "C:\\prog\\blergh1\\target\\thumbv7em-none-eabi\\debug\\build\\apache-nimble-1b316b1bda9c7042\\out\\ba62fddf2a5ea91f-ble_ll_hci_supp_cmd.o" "-c" "../mynewt-nimble/nimble/controller/src\\ble_ll_hci_supp_cmd.c"

The errors seemed to get fixed
when I downloaded a
[newer ARM GCC toolchain](https://developer.arm.com/downloads/-/arm-gnu-toolchain-downloads)
(unfortunately, 250MB archive,
unpacking to another ~1GB on disk...)
and used the GCC tools from it.

See the `b.bat` script for what worked on my machine.
