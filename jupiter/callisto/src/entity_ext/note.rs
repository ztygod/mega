use sea_orm::entity::prelude::*;

use crate::notes::{Column, Entity};

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Project,
    OrganizationMemberships,
    Comment,
    FollowUp,
    Permission,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Project => Entity::belongs_to(crate::projects::Entity)
                .from(Column::ProjectId)
                .to(crate::projects::Column::Id)
                .into(),
            Self::OrganizationMemberships => {
                Entity::belongs_to(crate::organization_memberships::Entity)
                    .from(Column::OrganizationMembershipId)
                    .to(crate::organization_memberships::Column::Id)
                    .into()
            }
            Self::Comment => Entity::has_many(crate::comments::Entity).into(),
            Self::FollowUp => Entity::has_many(crate::follow_ups::Entity).into(),
            Self::Permission => Entity::has_many(crate::permissions::Entity).into(),
        }
    }
}

impl Related<crate::comments::Entity> for crate::notes::Entity {
    fn to() -> RelationDef {
        Entity::has_many(crate::comments::Entity).into()
    }
}
