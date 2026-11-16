// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-compositor-effects: compositor effects component model.
//!
//! Real implementation: effects, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Effect {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompositorEffectsError {
    EmptyEffectName,
    DuplicateEffect(String),
    EmptyEventSubject,
}

impl fmt::Display for CompositorEffectsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyEffectName => write!(f, "effect name must not be empty"),
            Self::DuplicateEffect(id) => write!(f, "duplicate effect {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for CompositorEffectsError {}

pub struct CompositorEffects {
    pub name: String,
    effects: BTreeMap<String, Effect>,
    events: BTreeMap<String, Event>,
}

impl CompositorEffects {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            effects: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_effect(&mut self, item: Effect) -> Result<(), CompositorEffectsError> {
        if item.name.trim().is_empty() {
            return Err(CompositorEffectsError::EmptyEffectName);
        }
        let id = item.id.clone();
        if self.effects.contains_key(&id) {
            return Err(CompositorEffectsError::DuplicateEffect(id.clone()));
        }
        self.effects.insert(id, item);
        Ok(())
    }

    pub fn effect_count(&self) -> usize {
        self.effects.len()
    }

    pub fn get_effect(&self, id: &str) -> Option<&Effect> {
        self.effects.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), CompositorEffectsError> {
        if event.subject.trim().is_empty() {
            return Err(CompositorEffectsError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_effect(id: &str, name: &str) -> Effect {
        Effect {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = CompositorEffects::new("CompositorEffects");
        assert_eq!(m.name, "CompositorEffects");
        assert_eq!(m.effect_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_effect() {
        let mut m = CompositorEffects::new("CompositorEffects");
        m.add_effect(sample_effect("a1", "Alpha")).unwrap();
        assert_eq!(m.effect_count(), 1);
        assert!(m.get_effect("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = CompositorEffects::new("CompositorEffects");
        assert!(m.add_effect(sample_effect("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = CompositorEffects::new("CompositorEffects");
        m.add_effect(sample_effect("x", "One")).unwrap();
        assert!(m.add_effect(sample_effect("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = CompositorEffects::new("CompositorEffects");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
