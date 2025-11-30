use sea_orm::entity::prelude::*;

use crate::permissions::{Column, Entity};

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Note,
    User,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Note => Entity::belongs_to(crate::notes::Entity)
                .from(Column::SubjectId)
                .to(crate::notes::Column::Id)
                .into(),
            Self::User => Entity::has_many(crate::user_table::Entity).into(),
        }
    }
}

impl Related<crate::notes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Note.def()
    }
}
