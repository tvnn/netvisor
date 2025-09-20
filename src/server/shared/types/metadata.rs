use serde::Serialize;
use strum::IntoDiscriminant;
use std::fmt::Display;

#[derive(Serialize, Debug, Clone)]
pub struct MetadataRegistry {
    pub service_types: Vec<TypeMetadata>,
    pub subnet_types: Vec<TypeMetadata>,
    pub edge_types: Vec<TypeMetadata>,
    pub entities: Vec<EntityMetadata>
}

#[derive(Serialize, Debug, Clone)]
pub struct TypeMetadata {
    pub id: String,
    pub display_name: Option<&'static str>,
    pub description: Option<&'static str>,
    pub category: Option<&'static str>,
    pub icon: Option<&'static str>,
    pub color: Option<&'static str>,
    pub metadata: Option<serde_json::Value>,
}

#[derive(Serialize, Debug, Clone)]
pub struct EntityMetadata {
    pub id: String,
    pub color: &'static str,
    pub icon: &'static str
}

pub trait MetadataProvider<T>: IntoDiscriminant
where 
    <Self as IntoDiscriminant>::Discriminant: Display
{
    fn id(&self) -> String {
        self.discriminant().to_string()
    }
    fn to_metadata(&self) -> T;
}

pub trait EntityMetadataProvider: MetadataProvider<EntityMetadata> + IntoDiscriminant
where 
    <Self as IntoDiscriminant>::Discriminant: Display
{
    fn color(&self) -> &'static str;
    fn icon(&self) -> &'static str;
}

// TypeMetadataProvider now extends EntityMetadataProvider
pub trait TypeMetadataProvider: EntityMetadataProvider + MetadataProvider<TypeMetadata> + IntoDiscriminant
where 
    <Self as IntoDiscriminant>::Discriminant: Display
{
    fn display_name(&self) -> &'static str;
    fn description(&self) -> &'static str {
        ""
    }
    fn category(&self) -> &'static str {
        ""
    }
    fn metadata(&self) -> serde_json::Value {
        serde_json::json!({})
    }
}

// Blanket implementation for EntityMetadata
impl<T> MetadataProvider<EntityMetadata> for T 
where 
    T: EntityMetadataProvider + IntoDiscriminant,
    <T as IntoDiscriminant>::Discriminant: Display
{
    fn to_metadata(&self) -> EntityMetadata {
        EntityMetadata { 
            id: self.id(), 
            color: self.color(),
            icon: self.icon()
        }
    }
}

// Blanket implementation for TypeMetadata
impl<T> MetadataProvider<TypeMetadata> for T 
where 
    T: TypeMetadataProvider + IntoDiscriminant,
    <T as IntoDiscriminant>::Discriminant: Display
{
    fn to_metadata(&self) -> TypeMetadata {
        let id = <T as MetadataProvider<TypeMetadata>>::id(self);
        let display_name = self.display_name();
        let description = self.description();
        let category = self.category();
        let icon = self.icon();
        let color = self.color();
        let metadata = self.metadata();

        TypeMetadata {
            id,
            display_name: (!display_name.is_empty()).then_some(display_name),
            description: (!description.is_empty()).then_some(description),
            category: (!category.is_empty()).then_some(category),
            icon: (!icon.is_empty()).then_some(icon),
            color: (!color.is_empty()).then_some(color),
            metadata: (!metadata.as_object().map_or(false, |obj| obj.is_empty())).then_some(metadata)
        }
    }
}