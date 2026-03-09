use uuid::Uuid;

use crate::error::KernelError;

#[async_trait::async_trait]
pub(crate) trait TransferRecord {
    async fn transfer_record(
        &self,
        record_identifier: &Uuid,
        previous_workspace_identifier: &Uuid,
        target_workspace_identifier: &Uuid,
    ) -> Result<(), KernelError>;
}

#[async_trait::async_trait]
pub(crate) trait DuplicateRecord {
    async fn duplicate_record(
        &self,
        record_identifier: &Uuid,
        previous_workspace_identifier: &Uuid,
        target_workspace_identifier: &Uuid,
    ) -> Result<(), KernelError>;
}
