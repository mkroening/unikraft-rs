pub enum ExitMode {
    Halt,
    Restart,
    Crash,
}

impl From<ExitMode> for raw::ukplat_gstate {
    fn from(value: ExitMode) -> Self {
        match value {
            ExitMode::Halt => Self::UKPLAT_HALT,
            ExitMode::Restart => Self::UKPLAT_RESTART,
            ExitMode::Crash => Self::UKPLAT_CRASH,
        }
    }
}

pub fn exit(mode: ExitMode) -> ! {
    unsafe { raw::ukplat_terminate(mode.into()) }
}

// From `unikraft/include/uk/plat/bootstrap.h`
#[allow(non_camel_case_types)]
mod raw {
    #[repr(C)]
    pub enum ukplat_gstate {
        UKPLAT_HALT,
        UKPLAT_RESTART,
        _UKPLAT_SUSPEND,
        UKPLAT_CRASH,
    }

    extern "C" {
        /// Terminates this guest
        /// @param request Specify the type of shutdown
        ///                Invalid requests are mapped to UKPLAT_CRASH
        pub fn ukplat_terminate(request: ukplat_gstate) -> !;
    }
}
