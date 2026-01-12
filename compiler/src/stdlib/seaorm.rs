// SeaORM Standard Library
// Generiert SeaORM Entity-Code aus VelinScript Structs

use crate::parser::ast::*;

pub struct SeaORMStdlib;

impl SeaORMStdlib {
    /// Generiert SeaORM Entity aus VelinScript Struct
    pub fn generate_entity(struct_def: &Struct) -> String {
        let mut code = format!(
            "use sea_orm::entity::prelude::*;\n\n#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]\n#[sea_orm(table_name = \"{}\")]\npub struct Model {{\n",
            struct_def.name.to_lowercase()
        );

        // Felder generieren
        for field in &struct_def.fields {
            let rust_type = Self::velin_to_seaorm_type(&field.field_type);
            let column_name = Self::field_name_to_column(&field.name);
            
            code.push_str(&format!("    #[sea_orm(primary_key)]\n",));
            code.push_str(&format!("    pub {}: {},\n", field.name, rust_type));
        }

        code.push_str("}\n\n");

        // ActiveModel generieren
        code.push_str("#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]\npub enum Relation {}\n\n");
        code.push_str("impl ActiveModelBehavior for ActiveModel {}\n");

        code
    }

    /// Generiert SeaORM Query-Code für db.find()
    pub fn generate_find_code(entity_name: &str, id_expr: &str) -> String {
        format!(
            "{}::Entity::find_by_id({}).one(&db).await",
            entity_name, id_expr
        )
    }

    /// Generiert SeaORM Query-Code für db.findAll()
    pub fn generate_find_all_code(entity_name: &str) -> String {
        format!(
            "{}::Entity::find().all(&db).await",
            entity_name
        )
    }

    /// Generiert SeaORM Query-Code für db.save()
    pub fn generate_save_code(entity_name: &str, var_name: &str) -> String {
        format!(
            "{{\n        let active_model: {}::ActiveModel = {}.into();\n        active_model.insert(&db).await\n    }}",
            entity_name, var_name
        )
    }

    /// Generiert SeaORM Query-Code für db.update()
    pub fn generate_update_code(entity_name: &str, id_expr: &str, var_name: &str) -> String {
        format!(
            "{{\n        let mut active_model: {}::ActiveModel = {}::Entity::find_by_id({})\n            .one(&db)\n            .await\n            .unwrap()\n            .unwrap()\n            .into();\n        // Update fields from {}\n        active_model.update(&db).await\n    }}",
            entity_name, entity_name, id_expr, var_name
        )
    }

    /// Generiert SeaORM Query-Code für db.delete()
    pub fn generate_delete_code(entity_name: &str, id_expr: &str) -> String {
        format!(
            "{}::Entity::delete_by_id({}).exec(&db).await",
            entity_name, id_expr
        )
    }

    /// Generiert SeaORM Database Connection Setup
    pub fn generate_db_setup(database_url: &str) -> String {
        format!(
            "use sea_orm::{{Database, DatabaseConnection}};\n\npub async fn create_db_connection() -> Result<DatabaseConnection, sea_orm::DbErr> {{\n    Database::connect(\"{}\").await\n}}",
            database_url
        )
    }

    /// Generiert SeaORM Migration-Code
    pub fn generate_migration_code() -> String {
        "use sea_orm_migration::prelude::*;\n\npub struct Migration;\n\nimpl MigrationName for Migration {\n    fn name(&self) -> &str {\n        \"migration\"\n    }\n}\n\n#[async_trait::async_trait]\nimpl MigrationTrait for Migration {\n    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {\n        // Create tables\n        Ok(())\n    }\n\n    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {\n        // Drop tables\n        Ok(())\n    }\n}".to_string()
    }

    /// Konvertiert VelinScript Type zu SeaORM Type
    fn velin_to_seaorm_type(velin_type: &Type) -> String {
        match velin_type {
            Type::String => "String".to_string(),
            Type::Number => "f64".to_string(),
            Type::Boolean => "bool".to_string(),
            Type::List(ref inner) => format!("Vec<{}>", Self::velin_to_seaorm_type(inner)),
            Type::Named(ref name) => name.clone(),
            _ => "String".to_string(),
        }
    }

    /// Konvertiert Feldname zu Column-Name (snake_case)
    fn field_name_to_column(field_name: &str) -> String {
        // Einfache Konvertierung: camelCase -> snake_case
        let mut result = String::new();
        for (i, c) in field_name.chars().enumerate() {
            if c.is_uppercase() && i > 0 {
                result.push('_');
            }
            result.push(c.to_lowercase().next().unwrap());
        }
        result
    }

    /// Generiert Relationship-Code (One-to-Many)
    pub fn generate_one_to_many(entity1: &str, entity2: &str, foreign_key: &str) -> String {
        format!(
            "impl RelationTrait for Relation {{\n    fn def(&self) -> RelationDef {{\n        match self {{\n            Relation::{} => Entity::belongs_to({}::Entity)\n                .from({}::Column::{})\n                .to({}::Column::Id)\n                .into(),\n        }}\n    }}\n}}",
            entity2, entity2, entity1, foreign_key, entity2
        )
    }

    /// Generiert Query-Builder für komplexe Abfragen
    pub fn generate_query_builder(entity_name: &str) -> String {
        format!(
            "{}::Entity::find()\n    .filter(/* conditions */)\n    .order_by_asc(/* column */)\n    .limit(/* limit */)\n    .all(&db)\n    .await",
            entity_name
        )
    }
}
