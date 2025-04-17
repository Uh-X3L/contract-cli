// src/lib.rs
pub mod contract;
pub mod db;
pub mod migration_files;
pub mod utils;
pub use contract::Contract;
pub use db::establish_connection;

#[cfg(feature = "private")]
pub mod private_logic {
    pub fn secret_feature() {
        println!("This is private logic only available in private builds.");
    }
}
