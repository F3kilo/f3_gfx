use crate::back::man_scene::SceneId;
use crate::managers::scene::SceneManager;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

pub struct UniqueScene {
    id: SceneId,
    manager: SceneManager,
}

impl UniqueScene {
    pub fn new(mut manager: SceneManager) -> Self {
        let id = manager.create_scene();
        Self { manager, id }
    }

    pub fn get_id(&self) -> SceneId {
        self.id
    }
}

impl Drop for UniqueScene {
    fn drop(&mut self) {
        self.manager.drop_scene(self.id);
    }
}

impl fmt::Debug for UniqueScene {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Scene #{:?}", self.id)
    }
}

impl PartialEq for UniqueScene {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for UniqueScene {}

impl Hash for UniqueScene {
    fn hash<H: Hasher>(&self, state: &mut H) {
        Hash::hash(&self.id, state)
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Scene {
    unique: Arc<UniqueScene>,
}

impl Scene {
    pub fn new(manager: SceneManager) -> Self {
        Self {
            unique: Arc::new(UniqueScene::new(manager)),
        }
    }

    pub fn get_id(&self) -> SceneId {
        self.unique.get_id()
    }
}
