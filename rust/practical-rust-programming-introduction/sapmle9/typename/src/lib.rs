pub use typename_derive::TypeName;

pub trait TypeName {
    fn type_name(&self) -> &str;
}
