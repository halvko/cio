use async_trait::async_trait;
use macros::db;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::airtable::{AIRTABLE_BASE_ID_SWAG, AIRTABLE_SWAG_INVENTORY_ITEMS_TABLE};
use crate::core::UpdateAirtableRecord;
use crate::db::Database;
use crate::schema::swag_invetory_items;

#[db {
    new_struct_name = "SwagInventoryItem",
    airtable_base_id = "AIRTABLE_BASE_ID_SWAG",
    airtable_table = "AIRTABLE_SWAG_INVENTORY_ITEMS_TABLE",
    match_on = {
        "item" = "String",
        "size" = "String",
    },
}]
#[derive(Debug, Insertable, AsChangeset, PartialEq, Clone, JsonSchema, Deserialize, Serialize)]
#[table_name = "swag_invetory_items"]
pub struct NewSwagInventoryItem {
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub name: String,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub size: String,
    #[serde(default)]
    pub current_stock: i32,
    #[serde(default, skip_serializing_if = "String::is_empty")]
    pub item: String,
    #[serde(
        default,
        skip_serializing_if = "String::is_empty",
        serialize_with = "airtable_api::barcode_format_as_string::serialize",
        deserialize_with = "airtable_api::barcode_format_as_string::deserialize"
    )]
    pub barcode: String,

    /// This is populated by Airtable.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub link_to_item: Vec<String>,
}

/// Implement updating the Airtable record for a SwagInventoryItem.
#[async_trait]
impl UpdateAirtableRecord<SwagInventoryItem> for SwagInventoryItem {
    async fn update_airtable_record(&mut self, record: SwagInventoryItem) {
        if !reccord.link_to_item.is_empty() {
            self.link_to_item = record.link_to_item;
        }

        // This is a funtion in Airtable so we can't update it.
        self.name = "".to_string();
    }
}

/// Sync software vendors from Airtable.
pub async fn refresh_swag_invetory_items() {
    let db = Database::new();

    // Get all the records from Airtable.
    let results: Vec<airtable_api::Record<SwagInventoryItem>> = SwagInventoryItem::airtable().list_records(&SwagInventoryItem::airtable_table(), "Grid view", vec![]).await.unwrap();
    for inventory_item_record in results {
        let mut inventory_item: NewSwagInventoryItem = inventory_item_record.fields.into();

        db_inventory_item.update(&db).await;
    }
}

#[cfg(test)]
mod tests {
    use crate::finance::refresh_swag_invetory_items;

    #[ignore]
    #[tokio::test(flavor = "multi_thread")]
    async fn test_cron_swag_invetory_items() {
        refresh_swag_invetory_items().await;
    }
}