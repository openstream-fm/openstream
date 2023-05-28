use crate::Model;
use async_trait::async_trait;
use parking_lot::Mutex;
use std::any::Any;
use std::{any::TypeId, collections::HashMap, marker::PhantomData, sync::Arc};

static REGISTRY: Mutex<Option<Registry>> = Mutex::new(None);

#[macro_export]
macro_rules! register {
  ($type:ty) => {
    #[static_init::dynamic]
    static REGISTRATION: () = $crate::registry::Registry::global().register::<$type>();
  };
}

#[derive(Clone)]
pub struct Registry {
  inner: Arc<Mutex<HashMap<TypeId, RegistryItem>>>,
}

impl Registry {
  pub fn global() -> Self {
    let mut lock = REGISTRY.lock();
    if lock.is_some() {
      lock.as_ref().unwrap().clone()
    } else {
      let registry = Self::new();
      *lock = Some(registry.clone());
      registry
    }
  }

  pub fn new() -> Self {
    Self {
      inner: Arc::new(Mutex::new(HashMap::new())),
    }
  }

  pub fn register<M: Model + 'static>(&self) {
    let wrapper: ModelWrapper<M> = ModelWrapper { model: PhantomData };
    let type_id = Any::type_id(&wrapper);
    let item = RegistryItem {
      model: Arc::new(wrapper),
    };
    let mut map = self.inner.lock();
    map.insert(type_id, item);
  }

  pub async fn ensure_collections(&self) -> Result<(), mongodb::error::Error> {
    let items = {
      let lock = self.inner.lock();
      let items = lock.values().cloned().collect::<Vec<_>>();
      items
    };

    for item in items {
      item.ensure_collection().await?;
    }
    Ok(())
  }
}

impl Default for Registry {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Clone)]
pub struct RegistryItem {
  model: Arc<dyn DynModelWrapper>,
}

impl RegistryItem {
  pub async fn ensure_collection(&self) -> Result<(), mongodb::error::Error> {
    self.model.ensure_collection().await
  }
}

#[async_trait]
trait DynModelWrapper: Send + Sync + 'static {
  async fn ensure_collection(&self) -> Result<(), mongodb::error::Error>;
}

#[derive(Debug, Clone)]
struct ModelWrapper<M: Model + 'static> {
  model: PhantomData<M>,
}

#[async_trait]
impl<M: Model + 'static> DynModelWrapper for ModelWrapper<M> {
  async fn ensure_collection(&self) -> Result<(), mongodb::error::Error> {
    M::ensure_collection().await
  }
}
