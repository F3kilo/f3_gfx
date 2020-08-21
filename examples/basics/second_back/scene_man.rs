use f3_gfx::back::man_scene::{ManageScenes, SceneId};
use slog::Logger;
use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};

static SCENE_ID_COUNTER: AtomicU64 = AtomicU64::new(0);

pub struct SecondBackSceneManager {
    scenes: HashSet<SceneId>,
    logger: Logger,
}

impl SecondBackSceneManager {
    pub fn new(logger: Logger) -> SecondBackSceneManager {
        Self {
            scenes: HashSet::new(),
            logger,
        }
    }
}

impl ManageScenes for SecondBackSceneManager {
    fn create_scene(&mut self) -> SceneId {
        let id = SCENE_ID_COUNTER.fetch_add(1, Ordering::Relaxed).into();
        info!(self.logger, "SecondBackSceneManager loads: {:?}", id);
        self.scenes.insert(id);
        id
    }

    fn drop_scene(&mut self, id: SceneId) -> bool {
        info!(self.logger, "SecondBackSceneManager drops: {:?}", id);
        self.scenes.remove(&id)
    }

    fn contains(&self, id: SceneId) -> bool {
        info!(
            self.logger,
            "SecondBackSceneManager checks containing: {:?}", id
        );
        self.scenes.contains(&id)
    }

    fn ids(&self) -> Vec<SceneId> {
        info!(self.logger, "SecondBackSceneManager return ids:");
        self.scenes.iter().cloned().collect()
    }
}
