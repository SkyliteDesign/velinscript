
pub struct MongoDbStdlib;

impl MongoDbStdlib {
    pub fn generate_connect_code(url: &str) -> String {
        format!(
            "mongodb::Client::with_uri_str({}).await.map_err(|e| e.to_string())",
            url
        )
    }

    pub fn generate_database_code(client: &str, name: &str) -> String {
        format!("{}.database({})", client, name)
    }

    pub fn generate_collection_code(db: &str, name: &str) -> String {
        format!("{}.collection::<serde_json::Value>({})", db, name)
    }

    pub fn generate_insert_one_code(collection: &str, doc: &str) -> String {
        format!(
            "{}.insert_one({}, None).await.map(|r| r.inserted_id.to_string()).map_err(|e| e.to_string())",
            collection, doc
        )
    }

    pub fn generate_find_code(collection: &str, filter: &str) -> String {
        format!(
            "{}.find({}, None).await.map(|cursor| cursor.collect::<Result<Vec<_>, _>>().await.unwrap_or_default()).map_err(|e| e.to_string())",
            collection, filter
        )
    }

    pub fn generate_find_one_code(collection: &str, filter: &str) -> String {
        format!(
            "{}.find_one({}, None).await.map_err(|e| e.to_string())",
            collection, filter
        )
    }

    pub fn generate_update_one_code(collection: &str, filter: &str, update: &str) -> String {
        format!(
            "{}.update_one({}, {}, None).await.map(|r| r.modified_count > 0).map_err(|e| e.to_string())",
            collection, filter, update
        )
    }

    pub fn generate_delete_one_code(collection: &str, filter: &str) -> String {
        format!(
            "{}.delete_one({}, None).await.map(|r| r.deleted_count > 0).map_err(|e| e.to_string())",
            collection, filter
        )
    }

    pub fn generate_aggregate_code(collection: &str, pipeline: &str) -> String {
        format!(
            "{}.aggregate({}, None).await.map(|cursor| cursor.collect::<Result<Vec<_>, _>>().await.unwrap_or_default()).map_err(|e| e.to_string())",
            collection, pipeline
        )
    }

    pub fn generate_create_index_code(collection: &str, keys: &str, unique: bool) -> String {
        let unique_str = if unique { "true" } else { "false" };
        format!(
            "{{
                let index_model = mongodb::IndexModel::builder()
                    .keys({})
                    .options(mongodb::options::IndexOptions::builder().unique({}).build())
                    .build();
                {}.create_index(index_model, None).await.map(|_| ()).map_err(|e| e.to_string())
            }}",
            keys, unique_str, collection
        )
    }
}
