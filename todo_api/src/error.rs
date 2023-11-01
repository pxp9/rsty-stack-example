// from: https://github.com/rust-awesome-app/template-app-base/blob/main/src-tauri/src/error.rs

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Surreal(#[from] surrealdb::error::Db),

    #[error(transparent)]
    IO(#[from] std::io::Error),
}
