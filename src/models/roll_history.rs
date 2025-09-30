use core::fmt;

use leptos::*;
use crate::models::dice::DiceRollResult;
use chrono::{DateTime, Utc};

// Individual entry in the dice roll history
#[derive(Clone, Debug)]
pub struct DiceHistoryEntry {
    pub timestamp: DateTime<Utc>,
    pub roll_results: Vec<DiceRollResult>,
}

impl DiceHistoryEntry {
    pub fn new(roll_results: Vec<DiceRollResult>) -> Self {
        Self {
            roll_results,
            timestamp: Utc::now(),
        }
    }
    // pub get_timestamp() -> &str {
    //     self.timestamp.convert_to_system_timezone().format("%HH:%M:%S")
    // }
}

impl fmt::Display for DiceHistoryEntry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}", self.timestamp.format("%HH:%M:%S"), self.roll_results
        .iter()
        .map(|r| r.to_string())
        .collect::<Vec<_>>()
        .join(", "))
    }
}

// Global store for dice roll history
#[derive(Clone)]
pub struct DiceHistoryStore {
    history: RwSignal<Vec<DiceHistoryEntry>>,
}

impl DiceHistoryStore {
    pub fn new() -> Self {
        Self {
            history: create_rw_signal(Vec::new()),
        }
    }
    
    pub fn add_roll(&self, results: Vec<DiceRollResult>) {
        let entry = DiceHistoryEntry::new(results);
        self.history.update(|h| h.push(entry));
    }
    
    pub fn clear(&self) {
        self.history.set(Vec::new());
    }
    
    pub fn get_history(&self) -> ReadSignal<Vec<DiceHistoryEntry>> {
        self.history.read_only()
    }
}

// Create a context provider for global access
pub fn provide_dice_history() -> DiceHistoryStore {
    let store = DiceHistoryStore::new();
    provide_context(store.clone());
    store
}

// Helper function to use the history store from any component
pub fn use_dice_history() -> DiceHistoryStore {
    use_context::<DiceHistoryStore>().expect("No DiceHistoryStore has been provided")
}