use sea_orm::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
}

impl Model {
    pub async fn new(db: &DatabaseConnection, id: i32) -> anyhow::Result<()> {
        let user = ActiveModel {
            id: sea_orm::ActiveValue::Set(id)
        };
        Entity::insert(user).exec(db).await?;
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::categories::Entity")]
    Categories,
}

impl Related<super::categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Categories.def()
    }
}




impl ActiveModelBehavior for ActiveModel {}