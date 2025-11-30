use sea_orm::entity::prelude::*;

use crate::follow_ups::{Column, Entity};

#[derive(Copy, Clone, Debug, EnumIter)]

pub enum Relation {
    Note,
    // Comment,
    // Post,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Note => Entity::belongs_to(crate::notes::Entity)
                .from(Column::SubjectId)
                .to(crate::notes::Column::Id)
                .into(),
        }
    }
}

impl Related<crate::notes::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Note.def()
    }
}
