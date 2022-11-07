use sea_orm::entity::prelude::*;

use serde::{Deserialize, Serialize};

use symbols::EntityFilter;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "best_selling_video_games")]
pub struct Model {
    pub rank: i8,
    #[sea_orm(primary_key, auto_increment = false)]
    pub name: String,
    pub sales: String,
    pub series: String,
    pub platforms: String,
    pub initial_release_date: String,
    pub developer: String,
    pub publisher: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl EntityFilter for Entity {}

impl PartialEq for Column {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Column::Rank, Column::Rank)
                | (Column::Name, Column::Name)
                | (Column::Sales, Column::Sales)
                | (Column::Series, Column::Series)
                | (Column::Platforms, Column::Platforms)
                | (Column::InitialReleaseDate, Column::InitialReleaseDate)
                | (Column::Developer, Column::Developer)
                | (Column::Publisher, Column::Publisher)
        )
    }
}
