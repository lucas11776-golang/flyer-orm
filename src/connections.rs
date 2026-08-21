use std::any::Any;
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, RwLock};

use crate::Result;
use crate::Executor;

pub(crate) struct Connections {
    cache: RwLock<HashMap<String, &'static str>>,
    connections: RwLock<HashMap<String, Arc<dyn Any + Send + Sync>>>,
}

impl Connections {
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            connections: RwLock::new(HashMap::new()),
        }
    }

    pub fn add<E: Executor + 'static>(&mut self, connection: impl Into<String>, executor: E) {
        self.connections
            .write()
            .unwrap()
            .insert(connection.into(), Arc::new(executor));
    }

    pub fn get<E: Executor + 'static>(&self, connection: &str) -> Arc<E> {
        self
            .connections
            .read()
            .unwrap()
            .get(connection)
            .cloned()
            .and_then(|any| any.downcast::<E>().ok())
            .unwrap_or_else(|| panic!("Connection '{connection}' not found or type mismatch"))
    }

    pub fn remove(&mut self, connection: &str) {
        self
            .connections
            .write()
            .unwrap()
            .remove(connection);
    }

    pub fn cache(&self, path: &str) -> Result<String> {
        if let Some(&cached) = self.cache.read().unwrap().get(path) {
            return Ok(String::from(cached));
        }

        let content = fs::read_to_string(path)?;
        let static_str: &'static str = Box::leak(content.into_boxed_str());

        let mut cache = self.cache.write().unwrap();

        Ok(String::from(*cache.entry(String::from(path)).or_insert(static_str)))
    }
}