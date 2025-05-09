mod bigNum;

mod tests;

pub mod codec;
pub use bigNum::BigNum;
pub use codec::encode;
pub use codec::parse;
pub use codec::Base;
