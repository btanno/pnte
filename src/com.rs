use windows::Win32::System::Com::*;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
pub enum CoInit {
    ApartmentThreaded,
    MultiThreaded,
}

#[inline]
pub fn co_initialize(object: CoInit) -> windows::core::Result<()> {
    let object = match object {
        CoInit::ApartmentThreaded => COINIT_APARTMENTTHREADED,
        CoInit::MultiThreaded => COINIT_MULTITHREADED,
    };
    unsafe {
        CoInitializeEx(None, object | COINIT_DISABLE_OLE1DDE).ok()?;
    }
    Ok(())
}

#[inline]
pub fn co_uninitialize() {
    unsafe {
        CoUninitialize();
    }
}
