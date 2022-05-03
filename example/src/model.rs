use chrono::NaiveDateTime;

use sea_orm::entity::prelude::*;

use serde::{Deserialize, Serialize};

use symbols_models::EntityFilter;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Deserialize, Serialize)]
#[sea_orm(table_name = "example")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub name: String,
    pub kind: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}

impl EntityFilter for Entity {}

impl PartialEq for Column {
    fn eq(&self, other: &Self) -> bool {
        matches!(
            (self, other),
            (Column::Name, Column::Name)
                | (Column::Kind, Column::Kind)
                | (Column::CreatedAt, Column::CreatedAt)
                | (Column::UpdatedAt, Column::UpdatedAt)
        )
    }
}
