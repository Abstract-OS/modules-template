pub mod contract;
mod dependencies;
pub mod error;
mod handlers;

#[cfg(test)]
#[cfg(not(target_arch = "wasm32"))]
mod tests;
