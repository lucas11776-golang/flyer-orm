use std::any::Any;
use std::collections::HashMap;
use std::fs;
use std::sync::{Arc, RwLock};

use crate::Result;
use crate::Executor;

pub(crate) struct Connections {
    cache: RwLock<HashMap<String, &'static str>>,
    connections: HashMap<String, Arc<dyn Any + Send + Sync>>,
}

impl Connections {
    pub fn new() -> Self {
        Self {
            cache: RwLock::new(HashMap::new()),
            connections: HashMap::new(),
        }
    }

    pub fn add<E: Executor + 'static>(&mut self, connection: impl Into<String>, executor: E) {
        self.connections
            .insert(connection.into(), Arc::new(executor));
    }

    pub fn get<E: Executor + 'static>(&self, connection: &str) -> &E {
        self.connections
            .get(connection)
            .unwrap()
            .downcast_ref::<E>()
            .unwrap()
    }

    pub fn remove(&mut self, connection: &str) {
        self.connections.remove(connection);
    }

    pub fn cache(&self, path: &str) -> Result<&'static str> {
        if let Some(&cached) = self.cache.read().unwrap().get(path) {
            return Ok(cached);
        }

        let content = fs::read_to_string(path)?;
        let static_str: &'static str = Box::leak(content.into_boxed_str());

        let mut cache = self.cache.write().unwrap();

        Ok(*cache.entry(path.to_string()).or_insert(static_str))
    }
}