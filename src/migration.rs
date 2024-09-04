use crate::pxolly::api::PxollyAPI;
use crate::pxolly::types::categories::Categories;
use crate::pxolly::types::params::ImportChatLocalIdsParams;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct ChatModel {
    chat_uid: u64,
    chat_id: String,
}

pub async fn run_migration_chat_ids(api: PxollyAPI) {
    if let Ok(file) = std::fs::File::open("chat.json") {
        log::info!("The file `chat.json` is exists... Running migration...");
        let models: Vec<ChatModel> = serde_json::from_reader(file).unwrap();
        if models.is_empty() {
            log::info!("The file is empty. Migrate is stopped");
            log::info!("Deleting a file...");
            if let Err(err) = std::fs::remove_file("chat.json") {
                log::error!("Failed deleting file: {}", err);
            }
            return;
        }

        log::info!("Content file: {:?}", models);
        let mut chat_local_ids = HashMap::new();
        for model in models {
            chat_local_ids.insert(model.chat_id, model.chat_uid);
        }
        let params = ImportChatLocalIdsParams { chat_local_ids };
        let result = api.callback().import_chat_local_ids(params).await;

        match result {
            Ok(_) => {
                log::info!("Successfully imported chat! Deleting a file...");
                if let Err(err) = std::fs::remove_file("chat.json") {
                    log::error!("Failed deleting file: {}", err);
                }
            }
            Err(e) => log::error!("Failed to import chat: {}", e),
        }
    }
}
