use windows::Win32::System::Console::{GenerateConsoleCtrlEvent, CTRL_C_EVENT};

#[ctor::ctor]
fn ctor() {
    unsafe {
        GenerateConsoleCtrlEvent(CTRL_C_EVENT, 0);
    }
}
