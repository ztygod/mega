use crate::storage::{
    base_storage::{BaseStorage, StorageConnector},
    conversation_storage::ConversationStorage,
    note_storage::NoteStorage,
};

#[derive(Clone)]
pub struct NoteService {
    pub note_storage: NoteStorage,
    pub conversation_storage: ConversationStorage,
}

impl NoteService {
    pub fn new(base_storage: BaseStorage) -> Self {
        Self {
            note_storage: NoteStorage {
                base: base_storage.clone(),
            },
            conversation_storage: ConversationStorage {
                base: base_storage.clone(),
            },
        }
    }

    pub fn mock() -> Self {
        let mock = BaseStorage::mock();
        Self {
            note_storage: NoteStorage { base: mock.clone() },
            conversation_storage: ConversationStorage { base: mock.clone() },
        }
    }
}
