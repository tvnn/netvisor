use super::domain::Entity;

/// Core repository operations
#[async_trait]
pub trait Repository<E: Entity>: Send + Sync {
    async fn create(&self, entity: E) -> Result<E>;
    async fn get(&self, id: &E::Id) -> Result<Option<E>>;
    async fn get_all(&self) -> Result<Vec<E>>;
    async fn update(&self, entity: E) -> Result<E>;
    async fn delete(&self, id: &E::Id) -> Result<()>;
    async fn exists(&self, id: &E::Id) -> Result<bool>;
}

/// Extended query capabilities
#[async_trait]
pub trait QueryRepository<E: Entity>: Repository<E> {
    type Filter: Send + Sync;
    type Sort: Send + Sync;
    
    async fn find(&self, filter: Self::Filter) -> Result<Vec<E>>;
    async fn find_one(&self, filter: Self::Filter) -> Result<Option<E>>;
    async fn find_with_sort(&self, filter: Self::Filter, sort: Self::Sort) -> Result<Vec<E>>;
    async fn count(&self, filter: Option<Self::Filter>) -> Result<usize>;
}