use sea_orm::entity::prelude::*;

use crate::user_table::{Column, Entity};

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Permission,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Permission => Entity::belongs_to(crate::permissions::Entity)
                .from(Column::Id)
                .to(crate::permissions::Column::UserId)
                .into(),
        }
    }
}

impl Related<crate::permissions::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Permission.def()
    }
}
