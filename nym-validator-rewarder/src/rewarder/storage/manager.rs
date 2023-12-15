// Copyright 2023 - Nym Technologies SA <contact@nymtech.net>
// SPDX-License-Identifier: GPL-3.0-only

use crate::rewarder::epoch::Epoch;
use sqlx::types::time::OffsetDateTime;
use sqlx::{Executor, Sqlite};
use tracing::{instrument, trace};

#[derive(Clone)]
pub(crate) struct StorageManager {
    pub(crate) connection_pool: sqlx::SqlitePool,
}

impl StorageManager {
    pub(crate) async fn load_last_rewarding_epoch(&self) -> Result<Option<Epoch>, sqlx::Error> {
        sqlx::query_as(
            r#"
                    SELECT id, start_time, end_time
                    FROM rewarding_epoch
                    ORDER BY id DESC
                    LIMIT 1
                "#,
        )
        .fetch_optional(&self.connection_pool)
        .await
    }

    pub(crate) async fn insert_rewarding_epoch(
        &self,
        epoch: Epoch,
        rewarding_budget: String,
        rewarding_tx: Option<String>,
        rewarding_error: Option<String>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO rewarding_epoch (id, start_time, end_time, budget, rewarding_tx, rewarding_error)
                VALUES (?, ?, ? ,?, ?, ?)
            "#,
            epoch.id,
            epoch.start_time,
            epoch.end_time,
            rewarding_budget,
            rewarding_tx,
            rewarding_error
        ).execute(&self.connection_pool).await?;

        Ok(())
    }

    pub(crate) async fn insert_rewarding_epoch_block_signing(
        &self,
        epoch: i64,
        total_voting_power_at_epoch_start: i64,
        num_blocks: i64,
        budget: String,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO epoch_block_signing (rewarding_epoch_id, total_voting_power_at_epoch_start, num_blocks, budget)
                VALUES (?, ?, ?, ?)
            "#,
            epoch,
            total_voting_power_at_epoch_start,
            num_blocks,
            budget,
        ).execute(&self.connection_pool).await?;

        Ok(())
    }

    pub(crate) async fn insert_rewarding_epoch_block_signing_reward(
        &self,
        epoch: i64,
        consensus_address: String,
        operator_account: String,
        amount: String,
        voting_power: i64,
        voting_power_share: String,
        signed_blocks: i32,
        signed_blocks_percent: String,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO block_signing_reward (
                    rewarding_epoch_id,
                    validator_consensus_address,
                    operator_account,
                    amount,
                    voting_power,
                    voting_power_share,
                    signed_blocks,
                    signed_blocks_percent
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)
            "#,
            epoch,
            consensus_address,
            operator_account,
            amount,
            voting_power,
            voting_power_share,
            signed_blocks,
            signed_blocks_percent,
        )
        .execute(&self.connection_pool)
        .await?;

        Ok(())
    }

    pub(crate) async fn insert_rewarding_epoch_credential_issuance(
        &self,
        epoch: i64,
        dkg_epoch_id: u32,
        total_issued_credentials: u32,
        budget: String,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO epoch_credential_issuance (rewarding_epoch_id, dkg_epoch_id, total_issued_credentials, budget)
                VALUES (?, ?, ?, ?)
            "#,
            epoch,
            dkg_epoch_id,
            total_issued_credentials,
            budget,
        ).execute(&self.connection_pool).await?;

        Ok(())
    }

    pub(crate) async fn insert_rewarding_epoch_credential_issuance_reward(
        &self,
        epoch: i64,
        operator_account: String,
        amount: String,
        api_endpoint: String,
        issued_partial_credentials: u32,
        issued_credentials_share: String,
        validated_issued_credentials: u32,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO credential_issuance_reward (
                    rewarding_epoch_id,
                    operator_account,
                    amount,
                    api_endpoint,
                    issued_partial_credentials,
                    issued_credentials_share,
                    validated_issued_credentials
                ) VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
            epoch,
            operator_account,
            amount,
            api_endpoint,
            issued_partial_credentials,
            issued_credentials_share,
            validated_issued_credentials,
        )
        .execute(&self.connection_pool)
        .await?;

        Ok(())
    }
}
