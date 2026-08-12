pub mod pure;
pub mod list_helpers;

#[cfg(feature = "python")]
pub mod python;

pub fn register(scope: &mut eval::EvalScope) {
    pure::register(scope);

    #[cfg(feature = "python")]
    python::register(scope);
}
