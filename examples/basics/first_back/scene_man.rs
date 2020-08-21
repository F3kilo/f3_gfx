use f3_gfx::back::man_scene::{ManageScenes, SceneId};
use slog::Logger;
use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};

static SCENE_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

pub struct FirstBackSceneManager {
    scenes: HashSet<SceneId>,
    logger: Logger,
}

impl FirstBackSceneManager {
    pub fn new(logger: Logger) -> FirstBackSceneManager {
        Self {
            scenes: HashSet::new(),
            logger,
        }
    }
}

impl ManageScenes for FirstBackSceneManager {
    fn create_scene(&mut self) -> SceneId {
        let id = SCENE_ID_COUNTER.fetch_add(1, Ordering::Relaxed).into();
        info!(self.logger, "FirstBackSceneManager loads: {:?}", id);
        self.scenes.insert(id);
        id
    }

    fn drop_scene(&mut self, id: SceneId) -> bool {
        info!(self.logger, "FirstBackSceneManager drops: {:?}", id);
        self.scenes.remove(&id)
    }

    fn contains(&self, id: SceneId) -> bool {
        info!(
            self.logger,
            "FirstBackSceneManager checks containing: {:?}", id
        );
        self.scenes.contains(&id)
    }

    fn ids(&self) -> Vec<SceneId> {
        info!(self.logger, "FirstBackSceneManager return ids:");
        self.scenes.iter().cloned().collect()
    }
}
