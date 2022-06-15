use sea_orm::{prelude::*, FromQueryResult, QuerySelect};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "categories")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub user_id: i32,
    pub name: String
}

impl Model {
    pub async fn new(db: &DatabaseConnection, user_id: i32, name: String) -> anyhow::Result<()> {
        let category = ActiveModel {
            name: sea_orm::ActiveValue::Set(name),
            user_id: sea_orm::ActiveValue::Set(user_id),
            ..Default::default()
        };
        Entity::insert(category).exec(db).await?;
        Ok(())
    }
    pub async fn users_categories(db: &DatabaseConnection, user_id: i32) -> anyhow::Result<Vec<CategoryItem>> {
        let categories = Entity::find()
            .select_only()
            .filter(Column::UserId.eq(user_id))
            .column(Column::Id)
            .column(Column::Name)
            .into_model::<CategoryItem>()
            .all(db)
            .await?;
        Ok(categories)
    }
}
#[derive(FromQueryResult)]
pub struct CategoryItem {
    id: i32,
    name: String
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Users,
    #[sea_orm(has_many = "super::tracks::Entity")]
    Tracks
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Users.def()
    }
}

impl Related<super::tracks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tracks.def()
    }
}



impl ActiveModelBehavior for ActiveModel {}