use std::{ptr::null_mut, sync::Arc};

use symphonia::core::audio::AmbisonicBFormat::S;

use crate::steamaudio::{
    bind::{
        IPLDirectSimulationFlags, IPLDirectSimulationFlags_IPL_DIRECTSIMULATIONFLAGS_AIRABSORPTION,
        IPLDirectSimulationFlags_IPL_DIRECTSIMULATIONFLAGS_DIRECTIVITY,
        IPLDirectSimulationFlags_IPL_DIRECTSIMULATIONFLAGS_DISTANCEATTENUATION,
        IPLDirectSimulationFlags_IPL_DIRECTSIMULATIONFLAGS_OCCLUSION,
        IPLDirectSimulationFlags_IPL_DIRECTSIMULATIONFLAGS_TRANSMISSION, IPLReflectionEffectType,
        IPLReflectionEffectType_IPL_REFLECTIONEFFECTTYPE_CONVOLUTION,
        IPLReflectionEffectType_IPL_REFLECTIONEFFECTTYPE_HYBRID,
        IPLReflectionEffectType_IPL_REFLECTIONEFFECTTYPE_PARAMETRIC,
        IPLReflectionEffectType_IPL_REFLECTIONEFFECTTYPE_TAN, IPLSimulationFlags,
        IPLSimulationFlags_IPL_SIMULATIONFLAGS_DIRECT,
        IPLSimulationFlags_IPL_SIMULATIONFLAGS_PATHING,
        IPLSimulationFlags_IPL_SIMULATIONFLAGS_REFLECTIONS, IPLSimulationSettings, IPLSimulator,
        iplSimulatorCreate, iplSimulatorSetSharedInputs,
    },
    c_api_objs::context::Context,
    errors::Status,
};

pub struct Simulator {
    context: Arc<Context>,
    simulator: IPLSimulator,
}

impl Simulator {
    pub fn new(context: Arc<Context>, sim_setting: SimulationSettings) -> Result<Self, Status> {
        let mut simulator = null_mut();
        let mut settings = IPLSimulationSettings {
            flags: sim_setting.flags.convert(),
            sceneType: sim_setting.scene_type,
            reflectionType: IPLReflectionEffectType::from(sim_setting.reflection_type),
            maxNumOcclusionSamples: sim_setting.max_num_occlusion_samples,
            maxDuration: sim_setting.max_duration,
            samplingRate: sim_setting.sampling_rate,
            frameSize: sim_setting.frame_size,
            maxOrder: sim_setting.max_order,
            maxNumRays: sim_setting.max_num_rays,
            maxNumSources: sim_setting.max_num_sources,
            numVisSamples: sim_setting.num_vis_samples,
            numThreads: sim_setting.num_threads,
            numDiffuseSamples: sim_setting.num_diffuse_samples,
            rayBatchSize: sim_setting.ray_batch_size,
            openCLDevice: null_mut(),
            tanDevice: null_mut(),
            radeonRaysDevice: null_mut(),
        };

        let status = unsafe { iplSimulatorCreate(context.as_raw(), &mut settings, &mut simulator) };

        Status::catch(Self { context, simulator }, Status::from(status))
    }

    pub fn as_raw(&self) -> IPLSimulator {
        self.simulator
    }

    pub fn as_raw_mut(&mut self) -> &mut IPLSimulator {
        &mut self.simulator
    }

    pub fn set_shared_inputs(&self, flags: SimulationFlags) {
        let shared_inputs = null_mut();
        unsafe {
            iplSimulatorSetSharedInputs(self.simulator, flags.convert(), shared_inputs);
        }
    }
}

pub struct SimulationSettings {
    pub flags: SimulationFlags,
    pub scene_type: u32,
    pub reflection_type: ReflectionEffectType,
    pub max_num_occlusion_samples: i32,
    pub num_diffuse_samples: i32,
    pub max_duration: f32,
    pub max_order: i32,
    pub max_num_sources: i32,
    pub num_threads: i32,
    pub ray_batch_size: i32,
    pub num_vis_samples: i32,
    pub sampling_rate: i32,
    pub frame_size: i32,
    pub max_num_rays: i32,
}

impl Default for SimulationSettings {
    fn default() -> Self {
        Self {
            flags: SimulationFlags {
                direct: false,
                reflections: false,
                pathing: false,
            },
            scene_type: 0,
            reflection_type: ReflectionEffectType::Hybrid,
            max_num_occlusion_samples: 1024,
            num_diffuse_samples: 1024,
            max_duration: 5.0f32,
            max_order: 16,
            max_num_sources: 64,
            num_threads: 4,
            ray_batch_size: 64,
            num_vis_samples: 32,
            sampling_rate: 44100,
            frame_size: 1024,
            max_num_rays: 128,
        }
    }
}

pub enum ReflectionEffectType {
    Convolution,
    Parametric,
    Hybrid,
    Tan,
}

impl From<ReflectionEffectType> for IPLReflectionEffectType {
    fn from(value: ReflectionEffectType) -> Self {
        match value {
            ReflectionEffectType::Convolution => {
                IPLReflectionEffectType_IPL_REFLECTIONEFFECTTYPE_CONVOLUTION
            }
            ReflectionEffectType::Hybrid => IPLReflectionEffectType_IPL_REFLECTIONEFFECTTYPE_HYBRID,
            ReflectionEffectType::Parametric => {
                IPLReflectionEffectType_IPL_REFLECTIONEFFECTTYPE_PARAMETRIC
            }
            ReflectionEffectType::Tan => IPLReflectionEffectType_IPL_REFLECTIONEFFECTTYPE_TAN,
        }
    }
}

pub enum DirectSimulationFlags {
    DISTANCEATTENUATION,
    AIRABSORPTION,
    DIRECTIVITY,
    OCCLUSION,
    TRANSMISSION,
}

impl From<DirectSimulationFlags> for IPLDirectSimulationFlags {
    fn from(value: DirectSimulationFlags) -> Self {
        match value {
            DirectSimulationFlags::AIRABSORPTION => {
                IPLDirectSimulationFlags_IPL_DIRECTSIMULATIONFLAGS_AIRABSORPTION
            }
            DirectSimulationFlags::DIRECTIVITY => {
                IPLDirectSimulationFlags_IPL_DIRECTSIMULATIONFLAGS_DIRECTIVITY
            }
            DirectSimulationFlags::DISTANCEATTENUATION => {
                IPLDirectSimulationFlags_IPL_DIRECTSIMULATIONFLAGS_DISTANCEATTENUATION
            }
            DirectSimulationFlags::OCCLUSION => {
                IPLDirectSimulationFlags_IPL_DIRECTSIMULATIONFLAGS_OCCLUSION
            }
            DirectSimulationFlags::TRANSMISSION => {
                IPLDirectSimulationFlags_IPL_DIRECTSIMULATIONFLAGS_TRANSMISSION
            }
        }
    }
}

pub struct SimulationFlags {
    direct: bool,
    reflections: bool,
    pathing: bool,
}

impl SimulationFlags {
    pub fn convert(&self) -> IPLSimulationFlags {
        let mut res: u32 = 0;
        if self.direct {
            res += IPLSimulationFlags_IPL_SIMULATIONFLAGS_DIRECT;
        }

        if self.reflections {
            res += IPLSimulationFlags_IPL_SIMULATIONFLAGS_REFLECTIONS;
        }

        if self.pathing {
            res += IPLSimulationFlags_IPL_SIMULATIONFLAGS_PATHING;
        }

        res
    }
}
