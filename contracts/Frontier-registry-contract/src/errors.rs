use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyExist = 1,
    AlreadyTitleExist = 2, // already title exist
    AlreadyDescriptionExist = 3 // already description exist
}