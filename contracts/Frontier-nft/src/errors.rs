use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum Error {
    AlreadyInit = 1,
    TokenAlreadyExist = 2,
    TokenNoExist = 3,
    NotAuthorized = 4,
    NotOwner = 5,
    NotNFT = 6
}