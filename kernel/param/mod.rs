//! Parameter management

mod parser;
pub use parser::ParamParser;

pub mod obs_param;

/// Parameter handle error
pub enum ParamHandleErr {
    /// Unknown parameter
    Unknown,
    /// Parameter too large
    ParameterTooLarge,
}
