use super::{domain::*, repository::Repository};

/// Generic service providing CRUD operations - this is all you need!
pub struct CrudService<E, R> 
where
    E: Entity,
    R: Repository<E>,
{
    pub repository: R,  // Make public for easy access
}

impl<E, R> CrudService<E, R>
where
    E: Entity,
    R: Repository<E>,
{
    pub fn new(repository: R) -> Self {
        Self { repository }
    }
    
    pub async fn create<Req>(&self, request: Req) -> Result<E>
    where
        Req: CreateRequest<E>,
    {
        let entity = request.into_entity();
        self.repository.create(entity).await
    }
    
    pub async fn get(&self, id: &E::Id) -> Result<Option<E>> {
        self.repository.get(id).await
    }
    
    pub async fn get_all(&self) -> Result<Vec<E>> {
        self.repository.get_all().await
    }
    
    pub async fn update<Req>(&self, id: &E::Id, request: Req) -> Result<E>
    where
        Req: UpdateRequest<E>,
    {
        let mut entity = self.repository.get(id).await?
            .ok_or_else(|| anyhow::anyhow!("Entity not found: {:?}", id))?;
        
        request.apply_to(&mut entity);
        self.repository.update(entity).await
    }
    
    pub async fn delete(&self, id: &E::Id) -> Result<()> {
        self.repository.delete(id).await
    }
    
    pub async fn exists(&self, id: &E::Id) -> Result<bool> {
        self.repository.exists(id).await
    }
}