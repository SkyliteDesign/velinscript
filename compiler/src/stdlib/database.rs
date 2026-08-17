// Standard Library für Database-Funktionalität
// Diese Funktionen werden vom Code Generator in Rust-Code transformiert
// Unterstützt sowohl SeaORM als auch sqlx

/// Database Standard Library Funktionen
pub struct DatabaseStdlib;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ORMType {
    SeaORM,
    Sqlx,
}

impl DatabaseStdlib {
    /// Transformiert VelinScript db.find() zu Rust-Code
    pub fn generate_find_code(entity: &str, id: &str, orm: ORMType) -> String {
        match orm {
            ORMType::SeaORM => {
                #[cfg(feature = "sea-orm")]
                {
                    format!("{}::Entity::find_by_id({}).one(&db).await", entity, id)
                }
                #[cfg(not(feature = "sea-orm"))]
                {
                    format!("db.find::<{}>({})", entity, id)
                }
            }
            ORMType::Sqlx => {
                format!("db.find::<{}>({})", entity, id)
            }
        }
    }

    /// Transformiert VelinScript db.findAll() zu Rust-Code
    pub fn generate_find_all_code(entity: &str, orm: ORMType) -> String {
        match orm {
            ORMType::SeaORM => {
                #[cfg(feature = "sea-orm")]
                {
                    format!("{}::Entity::find().all(&db).await", entity)
                }
                #[cfg(not(feature = "sea-orm"))]
                {
                    format!("db.find_all::<{}>().await", entity)
                }
            }
            ORMType::Sqlx => {
                format!("db.find_all::<{}>().await", entity)
            }
        }
    }

    /// Transformiert VelinScript db.save() zu Rust-Code
    #[allow(unused_variables)]
    pub fn generate_save_code(entity: &str, var_name: &str, orm: ORMType) -> String {
        match orm {
            ORMType::SeaORM => {
                #[cfg(feature = "sea-orm")]
                {
                    format!(
                        "{{\n        let active_model: {}::ActiveModel = {}.into();\n        active_model.insert(&db).await\n    }}",
                        entity, var_name
                    )
                }
                #[cfg(not(feature = "sea-orm"))]
                {
                    format!("db.save({}).await", var_name)
                }
            }
            ORMType::Sqlx => {
                format!("db.save({}).await", var_name)
            }
        }
    }

    /// Transformiert VelinScript db.update() zu Rust-Code
    #[allow(unused_variables)]
    pub fn generate_update_code(entity: &str, id: &str, var_name: &str, orm: ORMType) -> String {
        match orm {
            ORMType::SeaORM => {
                #[cfg(feature = "sea-orm")]
                {
                    format!(
                        "{{\n        let mut active_model: {}::ActiveModel = {}::Entity::find_by_id({})\n            .one(&db)\n            .await?\n            .ok_or_else(|| anyhow::anyhow!(\"Entity not found\"))?\n            .into();\n        // Update fields from {}\n        active_model.update(&db).await\n    }}",
                        entity, entity, id, var_name
                    )
                }
                #[cfg(not(feature = "sea-orm"))]
                {
                    format!("db.update({}, {}).await", var_name, id)
                }
            }
            ORMType::Sqlx => {
                format!("db.update({}, {}).await", var_name, id)
            }
        }
    }

    /// Transformiert VelinScript db.delete() zu Rust-Code
    pub fn generate_delete_code(entity: &str, id: &str, orm: ORMType) -> String {
        match orm {
            ORMType::SeaORM => {
                #[cfg(feature = "sea-orm")]
                {
                    format!("{}::Entity::delete_by_id({}).exec(&db).await", entity, id)
                }
                #[cfg(not(feature = "sea-orm"))]
                {
                    format!("db.delete::<{}>({}).await", entity, id)
                }
            }
            ORMType::Sqlx => {
                format!("db.delete::<{}>({}).await", entity, id)
            }
        }
    }

    /// Generiert Database Connection Setup basierend auf ORM
    pub fn generate_db_setup(database_url: &str, orm: ORMType) -> String {
        match orm {
            ORMType::SeaORM => {
                #[cfg(feature = "sea-orm")]
                {
                    format!(
                        "use sea_orm::{{Database, DatabaseConnection}};\n\npub async fn create_db_connection() -> Result<DatabaseConnection, sea_orm::DbErr> {{\n    Database::connect(\"{}\").await\n}}",
                        database_url
                    )
                }
                #[cfg(not(feature = "sea-orm"))]
                {
                    Self::generate_sqlx_setup(database_url)
                }
            }
            ORMType::Sqlx => Self::generate_sqlx_setup(database_url),
        }
    }

    fn generate_sqlx_setup(database_url: &str) -> String {
        format!(
            r#"use sqlx::{{PgPool, MySqlPool, SqlitePool}};

pub async fn create_db_connection() -> Result<PgPool, sqlx::Error> {{
    sqlx::PgPool::connect("{}").await
}}"#,
            database_url
        )
    }

    /// Transformiert VelinScript db.query() zu Rust-Code (Query Builder)
    pub fn generate_query_code(query: &str, orm: ORMType) -> String {
        match orm {
            ORMType::SeaORM => {
                #[cfg(feature = "sea-orm")]
                {
                    format!("sea_orm::Statement::from_sql_and_values(sea_orm::DatabaseBackend::Postgres, \"{}\", vec![])", query)
                }
                #[cfg(not(feature = "sea-orm"))]
                {
                    format!("sqlx::query(\"{}\")", query)
                }
            }
            ORMType::Sqlx => {
                format!("sqlx::query(\"{}\")", query)
            }
        }
    }

    /// Transformiert VelinScript db.transaction() zu Rust-Code
    pub fn generate_transaction_code(block: &str, orm: ORMType) -> String {
        match orm {
            ORMType::SeaORM => {
                #[cfg(feature = "sea-orm")]
                {
                    format!(
                        "db.transaction(|txn| Box::pin(async move {{\n        {}\n        Ok(())\n    }})).await",
                        block
                    )
                }
                #[cfg(not(feature = "sea-orm"))]
                {
                    format!("db.begin().await?;\n    {}\n    db.commit().await?", block)
                }
            }
            ORMType::Sqlx => {
                format!(
                    "let mut tx = db.begin().await?;\n    {}\n    tx.commit().await?",
                    block
                )
            }
        }
    }

    /// Verbesserte save() Methode - unterscheidet Insert vs Update
    #[allow(unused_variables)]
    pub fn generate_save_code_improved(
        entity: &str,
        var_name: &str,
        id_field: &str,
        orm: ORMType,
    ) -> String {
        match orm {
            ORMType::SeaORM => {
                #[cfg(feature = "sea-orm")]
                {
                    format!(
                        "{{\n        let active_model: {}::ActiveModel = {}.into();\n        if active_model.{}.is_set() {{\n            active_model.update(&db).await\n        }} else {{\n            active_model.insert(&db).await\n        }}\n    }}",
                        entity, var_name, id_field
                    )
                }
                #[cfg(not(feature = "sea-orm"))]
                {
                    format!("db.save({}).await", var_name)
                }
            }
            ORMType::Sqlx => {
                format!("db.save({}).await", var_name)
            }
        }
    }

    /// Generiert Entity-Code für SeaORM
    pub fn generate_seaorm_entity(struct_name: &str, fields: &[(String, String)]) -> String {
        let mut entity_code = format!(
            r#"#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "{}")]
pub struct Model {{
"#,
            struct_name.to_lowercase()
        );

        for (field_name, field_type) in fields {
            let rust_type = match field_type.as_str() {
                "string" => "String",
                "number" | "int" => "i64",
                "float" => "f64",
                "boolean" | "bool" => "bool",
                _ => field_type,
            };

            if field_name == "id" {
                entity_code.push_str(&format!(
                    "    #[sea_orm(primary_key)]\n    pub {}: {},\n",
                    field_name, rust_type
                ));
            } else {
                entity_code.push_str(&format!("    pub {}: {},\n", field_name, rust_type));
            }
        }

        entity_code.push_str("}\n");
        entity_code
    }

    /// Erkennt ORM-Type aus Config oder verwendet Default
    pub fn detect_orm(config_orm: Option<&str>) -> ORMType {
        match config_orm {
            Some("seaorm") | Some("sea-orm") => ORMType::SeaORM,
            Some("sqlx") => ORMType::Sqlx,
            _ => ORMType::Sqlx, // Default to sqlx for backward compatibility
        }
    }
}
