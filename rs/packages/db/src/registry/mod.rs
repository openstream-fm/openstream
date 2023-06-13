use crate::error::CheckCollectionError;
use crate::Model;
use async_trait::async_trait;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::collections::BTreeMap;
use std::{any::TypeId, collections::HashMap, marker::PhantomData, sync::Arc};

pub use paste::paste;

static REGISTRY: Lazy<Registry> = Lazy::new(Registry::new);

#[macro_export]
macro_rules! register {
  ($type:ty) => {
    $crate::registry::paste! {
      #[static_init::dynamic]
      static [<$type:snake:upper _REGISTRATION>]: () = $crate::registry::Registry::global().register::<$type>();
    }
  };
}

#[derive(Clone)]
pub struct Registry {
  inner: Arc<Mutex<HashMap<TypeId, RegistryItem>>>,
}

impl Registry {
  pub fn global() -> Self {
    REGISTRY.clone()
  }

  pub fn new() -> Self {
    Self {
      inner: Arc::new(Mutex::new(HashMap::new())),
    }
  }

  pub fn register<M: Model + 'static>(&self) {
    let wrapper: ModelWrapper<M> = ModelWrapper { model: PhantomData };
    let type_id = TypeId::of::<M>();
    let item = RegistryItem {
      model: Arc::new(wrapper),
    };
    let mut map = self.inner.lock();
    map.insert(type_id, item);
  }

  pub fn items(&self) -> Vec<RegistryItem> {
    let lock = self.inner.lock();
    let items = lock.values().cloned().collect::<Vec<_>>();
    items
  }

  pub async fn ensure_collections(&self) -> Result<(), mongodb::error::Error> {
    let items = self.items();

    for item in items {
      item.ensure_collection().await?;
    }
    Ok(())
  }

  pub async fn check_all(
    &self,
  ) -> BTreeMap<&'static str, Result<u64, crate::error::CheckCollectionError>> {
    let items = self.items();

    let map: BTreeMap<_, _> = items
      .into_iter()
      .map(|item| (item.cl_name(), item))
      .collect();

    let mut results = BTreeMap::new();

    for (cl_name, item) in map.into_iter() {
      let r = item.check_collection_documents().await;
      results.insert(cl_name, r);
    }

    results
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

  pub fn cl_name(&self) -> &'static str {
    self.model.cl_name()
  }

  pub async fn check_collection_documents(
    &self,
  ) -> Result<u64, crate::error::CheckCollectionError> {
    self.model.check_collection_documents().await
  }
}

#[async_trait]
trait DynModelWrapper: Send + Sync + 'static {
  fn cl_name(&self) -> &'static str;
  async fn ensure_collection(&self) -> Result<(), mongodb::error::Error>;
  async fn check_collection_documents(&self) -> Result<u64, CheckCollectionError>;
}

#[derive(Debug, Clone)]
struct ModelWrapper<M: Model + 'static> {
  model: PhantomData<&'static M>,
}

#[async_trait]
impl<M: Model + 'static> DynModelWrapper for ModelWrapper<M> {
  fn cl_name(&self) -> &'static str {
    M::CL_NAME
  }

  async fn ensure_collection(&self) -> Result<(), mongodb::error::Error> {
    M::ensure_collection().await
  }

  async fn check_collection_documents(&self) -> Result<u64, CheckCollectionError> {
    M::check_collection_documents().await
  }
}
