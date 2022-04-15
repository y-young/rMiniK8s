use serde::Serialize;
use std::sync::Arc;

use crate::{etcd, AppState};

use super::response::ErrResponse;

pub async fn etcd_put(
    app_state: Arc<AppState>,
    key: String,
    val: impl Serialize,
) -> Result<(), ErrResponse> {
    let mut client = app_state.get_client().await?;
    etcd::put(&mut client, key, val)
        .await
        .map_err(ErrResponse::from)?;
    Ok(())
}