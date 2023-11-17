use crate as sea_orm;
use sea_orm::DeriveEntityModel;


#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "cake", schema_name = "public")]
pub struct Model {
    #[sea_orm(primary_key)] //Entity Structure 참고 (컬럼별 설정값)
    pub id: i32,
    pub name: String,
    //pub name: Option<String>, // nullable일 때
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    //  다른 entity와의 관계 기입
    #[sea_orm(has_many = "super::fruit::Entity")]
    Fruit,
    #[sea_orm(
        has_many = "super::fruit::Entity",
        on_condition = r#"super::fruit::Column::Na`me.like("%tropical%")"#
    )]
    TropicalFruit,
    #[sea_orm(
        has_many = "super::fruit::Entity",
        condition_type = "any",
        on_condition = r#"super::fruit::Column::Name.like("%tropical%")"#
    )]
    OrTropicalFruit,
}

impl Related<super::fruit::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Fruit.def()
    }
}

impl Related<super::filling::Entity> for Entity {
    fn to() -> RelationDef {
        super::cake_filling::Relation::Filling.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::cake_filling::Relation::Cake.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
