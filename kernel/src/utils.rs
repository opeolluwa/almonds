use std::{env, str::FromStr};

use crate::{adapters::meta::RequestMeta, error::KernelError};

pub(crate) fn extract_req_meta(meta: &Option<RequestMeta>) -> Result<RequestMeta, KernelError> {
    let Some(meta) = meta else {
        return Err(KernelError::DbConnectError(
            "missing workspace identifier".into(),
        ));
    };

    Ok(meta.to_owned())
}

pub fn extract_env<T: FromStr>(env_key: &str) -> Result<T, KernelError> {
    let env = env::var(env_key)
        .map_err(|_| {
            log::error!("error fetching env {}", env_key);
            KernelError::EnvError(env_key.to_string())
        })?
        .parse::<T>()
        .map_err(|_| {
            log::error!("error parsing env due to");
            KernelError::EnvError("error parsing env".into())
        })?;

    Ok(env)
}
