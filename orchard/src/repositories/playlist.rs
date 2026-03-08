use std::sync::Arc;

use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder,
};
use uuid::Uuid;

use crate::adapters::pagination::PaginationParams;
use crate::adapters::playlist::{CreatePlaylistRequest, UpdatePlaylistRequest};
use crate::entities::playlists;
use crate::errors::database_error::DatabaseError;
use crate::repositories::base::Repository;

#[derive(Clone)]
pub struct PlaylistRepository {
    db_conn: Arc<DatabaseConnection>,
}

impl Repository for PlaylistRepository {
    fn init(db_conn: &Arc<DatabaseConnection>) -> Self {
        Self {
            db_conn: db_conn.to_owned(),
        }
    }
}
pub(crate) trait PlaylistRepositoryExt {
    async fn create_new(
        &self,
        request: &CreatePlaylistRequest,
        user_identifier: &Uuid,
    ) -> Result<Uuid, DatabaseError>;

    async fn update(
        &self,
        request: &UpdatePlaylistRequest,
        playlist_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> Result<playlists::Model, DatabaseError>;

    async fn delete(
        &self,
        playlist_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> Result<(), DatabaseError>;

    async fn fetch_one(
        &self,
        playlist_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> Result<Option<playlists::Model>, DatabaseError>;

    async fn fetch_many(
        &self,
        pagination: &PaginationParams,
        user_identifier: &Uuid,
    ) -> Result<Vec<playlists::Model>, DatabaseError>;
}

impl PlaylistRepositoryExt for PlaylistRepository {
    async fn create_new(
        &self,
        request: &CreatePlaylistRequest,
        user_identifier: &Uuid,
    ) -> Result<Uuid, DatabaseError> {
        let playlist = playlists::ActiveModel {
            identifier: Set(Uuid::new_v4()),
            name: Set(request.name.to_owned()),
            description: Set(request.description.to_owned()),
            user_identifier: Set(*user_identifier),
            ..Default::default()
        };

        let playlist = playlist.insert(self.db_conn.as_ref()).await?;

        Ok(playlist.identifier)
    }

    async fn update(
        &self,
        request: &UpdatePlaylistRequest,
        playlist_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> Result<playlists::Model, DatabaseError> {
        let Some(playlist) = self.fetch_one(playlist_identifier, user_identifier).await? else {
            return Err(DatabaseError::RecordNotFound);
        };

        let mut playlist: playlists::ActiveModel = playlist.into();

        if let Some(name) = &request.name {
            playlist.name = Set(name.to_owned());
        }

        if let Some(description) = &request.description {
            playlist.description = Set(Some(description.to_owned()));
        }
        let playlist = playlist.update(self.db_conn.as_ref()).await?;

        Ok(playlist)
    }

    async fn delete(
        &self,
        playlist_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> Result<(), DatabaseError> {
        let Some(playlist) = self.fetch_one(playlist_identifier, user_identifier).await? else {
            return Err(DatabaseError::RecordNotFound);
        };

        let playlist: playlists::ActiveModel = playlist.into();

        playlist.delete(self.db_conn.as_ref()).await?;

        Ok(())
    }

    async fn fetch_one(
        &self,
        playlist_identifier: &Uuid,
        user_identifier: &Uuid,
    ) -> Result<Option<playlists::Model>, DatabaseError> {
        let playlist = playlists::Entity::find()
            .filter(playlists::Column::Identifier.eq(playlist_identifier.to_owned()))
            .filter(playlists::Column::UserIdentifier.eq(user_identifier.to_owned()))
            .one(self.db_conn.as_ref())
            .await
            .map_err(DatabaseError::from)?;

        Ok(playlist)
    }

    async fn fetch_many(
        &self,
        _pagination: &PaginationParams,
        user_identifier: &Uuid,
    ) -> Result<Vec<playlists::Model>, DatabaseError> {
        let playlists = playlists::Entity::find()
            .filter(playlists::Column::UserIdentifier.eq(*user_identifier))
            .order_by_asc(playlists::Column::CreatedAt)
            .all(self.db_conn.as_ref())
            .await?;

        Ok(playlists)
    }
}
