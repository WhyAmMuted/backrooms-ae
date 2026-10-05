use std::{ptr::null_mut, sync::Arc};

use crate::steamaudio_module::{
    IPLScene, IPLSceneSettings, IPLSceneType_IPL_SCENETYPE_DEFAULT, SteamAudioErrors,
    basic::context::Context, catch_error, iplSceneCommit, iplSceneCreate, iplSceneRelease,
};

pub struct Scene {
    scene: IPLScene,
    context: Arc<Context>,
}

impl Scene {
    pub fn new(context: Arc<Context>) -> Result<Self, SteamAudioErrors> {
        let mut settings = IPLSceneSettings {
            type_: IPLSceneType_IPL_SCENETYPE_DEFAULT,
            closestHitCallback: None,
            anyHitCallback: None,
            batchedAnyHitCallback: None,
            embreeDevice: null_mut(),
            batchedClosestHitCallback: None,
            radeonRaysDevice: null_mut(),
            userData: null_mut(),
        };

        let mut scene = null_mut();
        let status = unsafe { iplSceneCreate(context.as_raw(), &mut settings, &mut scene) };
        catch_error(
            SteamAudioErrors::convert(status),
            Scene {
                scene: scene,
                context,
            },
        )
    }

    pub fn as_raw(&self) -> &IPLScene {
        &self.scene
    }

    pub fn commit(&self) {
        unsafe {
            iplSceneCommit(self.scene);
        }
    }
}

impl From<Scene> for IPLScene {
    fn from(value: Scene) -> Self {
        value.scene
    }
}

impl Drop for Scene {
    fn drop(&mut self) {
        unsafe {
            iplSceneRelease(&mut self.scene);
        }
    }
}
