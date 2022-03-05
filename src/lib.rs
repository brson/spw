#![cfg(any(target_arch = "bpf", doc))]

pub mod error {
    #[doc(inline)]
    pub use solana_program::program_error::ProgramError;

    pub type ProgramResult<T> = Result<T, ProgramError>;
}

pub mod log { }

pub mod address {
    pub struct Address;
}

pub mod account_info {
    pub struct AccountInfo;
}

pub mod instruction { }

pub mod cpi { }

pub mod sysvars { }

pub mod programs {
    pub mod system {
    }
}
