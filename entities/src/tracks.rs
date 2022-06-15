use chrono::{Utc, DateTime};
use sea_orm::{prelude::*, ActiveValue, QuerySelect, FromQueryResult};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "tracks")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = true)]
    pub id: i32,
    pub category_id: i32,
    #[sea_orm(primary_key)]
    pub saved_at: DateTime<Utc>,
    pub time: i32
}

impl Model {
    pub async fn new(db: &DatabaseConnection, category_id: i32, time: i32) -> anyhow::Result<()> {
        let saved_at = chrono::Utc::now();
        let track = ActiveModel {
            category_id: sea_orm::ActiveValue::Set(category_id),
            time: sea_orm::ActiveValue::Set(time),
            saved_at: ActiveValue::Set(saved_at),
            ..Default::default()
        };
        Entity::insert(track).exec(db).await?;
        Ok(())
    }
    pub async fn sum_of_period(db: &DatabaseConnection, category: i32, period: Period) -> anyhow::Result<String> {
        #[derive(FromQueryResult, Default)]
        struct Sum {
            sum: i32
        }
        let Sum {sum } = Entity::find()
            .filter(Column::CategoryId.eq(category))
            .filter(Column::SavedAt.lte(period.start))
            .filter(Column::SavedAt.gte(period.end))
            .select_only()
            .column_as(Column::Time.sum(), "sum")
            .into_model::<Sum>()
            .one(db)
            .await?
            .unwrap_or_default();
        Ok(sum.to_string())
    }
}

pub struct Period {
    start: DateTime<Utc>,
    end: DateTime<Utc>
}


#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::categories::Entity",
        from = "Column::CategoryId",
        to = "super::categories::Column::Id",
        on_update = "NoAction",
        on_delete = "NoAction"
    )]
    Categories,
}

impl Related<super::categories::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Categories.def()
    }
}


impl ActiveModelBehavior for ActiveModel {}