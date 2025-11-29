// use sea_orm::entity::prelude::*;

// use crate::notes::{Column, Entity};

// #[derive(Copy, Clone, Debug, EnumIter)]
// pub enum Relation {
//     Project,
//     Member,
//     Conversations,
// }

// impl RelationTrait for Relation {
//     fn def(&self) -> RelationDef {
//         match self {
//             Self::Conversations => Entity::belongs_to(crate::mega_conversation::Entity)
//                 .from(Column::)
//         }
//     }
// }
