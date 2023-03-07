use std::env::{self, VarError};

use anyhow::{Error, Result};

pub fn var(key: &str) -> Var<String> {
    Var(env::var(key).map_err(Error::from))
}

pub struct Var<T>(Result<T>);

impl<T> Var<T> {
    pub fn new(var: T) -> Self {
        Var(Ok(var))
    }
    pub fn or_missing(self, value: Self) -> Self {
        let result = self.0.or_else(|err: Error| match err.downcast_ref() {
            Some(VarError::NotPresent) => value.0,
            _ => Err(err),
        });
        Self(result)
    }

    pub fn and_then<F, U>(self, f: F) -> Var<U>
    where
        F: FnOnce(T) -> Result<U>,
    {
        let result = self.0.and_then(f);
        Var(result)
    }

    pub fn to_result(self) -> Result<T> {
        self.0
    }
}
