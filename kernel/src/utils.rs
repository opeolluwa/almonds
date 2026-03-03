use crate::{adapters::meta::RequestMeta, error::KernelError};

pub(crate) fn extract_req_meta(meta: &Option<RequestMeta>) -> Result<RequestMeta, KernelError> {
    let Some(meta) = meta else {
        return Err(KernelError::DbConnectError(
            "missing workspace identifier".into(),
        ));
    };

    Ok(meta.to_owned())
}
