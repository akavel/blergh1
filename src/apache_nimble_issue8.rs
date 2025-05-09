// see: https://github.com/benbrittain/apache-nimble-sys/issues/8

#[unsafe(no_mangle)]
pub extern "C" fn _sbrk() {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub extern "C" fn _write() {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub extern "C" fn _close() {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub extern "C" fn _lseek() {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub extern "C" fn _read() {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub extern "C" fn _fstat() {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub extern "C" fn _isatty() {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub extern "C" fn _exit() {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub extern "C" fn _open() {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub extern "C" fn _kill() {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub extern "C" fn _getpid() {
    unimplemented!()
}
