use std::{ptr::null_mut, sync::Arc};

use crate::steamaudio::{
    bind::{
        IPLCoordinateSpace3, IPLDirectEffectParams,
        IPLDirectSimulationFlags_IPL_DIRECTSIMULATIONFLAGS_OCCLUSION,
        IPLOcclusionType_IPL_OCCLUSIONTYPE_RAYCAST, IPLSimulationFlags_IPL_SIMULATIONFLAGS_DIRECT,
        IPLSimulationInputs, IPLSource, IPLSourceSettings, iplSourceCreate, iplSourceGetOutputs,
        iplSourceRelease, iplSourceSetInputs,
    },
    c_api_objs::phys::simulator::{SimulationFlags, Simulator},
    errors::Status,
};

pub struct Source {
    simulator: Arc<Simulator>,
    source: IPLSource,
}
pub type CoordinateSpace3D = IPLCoordinateSpace3;
impl Source {
    pub fn new(sim: Arc<Simulator>, sim_flags: SimulationFlags) -> Result<Self, Status> {
        let mut source = null_mut();

        let status = unsafe {
            iplSourceCreate(
                sim.as_raw(),
                &mut IPLSourceSettings {
                    flags: sim_flags.convert(),
                },
                &mut source,
            )
        };

        Status::catch(
            Self {
                simulator: sim,
                source: source,
            },
            Status::from(status),
        )
    }

    pub fn as_raw(&self) -> IPLSource {
        self.source
    }

    pub fn as_raw_mut(&mut self) -> &mut IPLSource {
        &mut self.source
    }

    pub fn set_inputs(&mut self, pos: CoordinateSpace3D, flags: SimulationFlags) {
        unsafe {
            iplSourceSetInputs(
                self.source,
                flags.convert(),
                &mut IPLSimulationInputs {
                    flags: IPLSimulationFlags_IPL_SIMULATIONFLAGS_DIRECT,
                    directFlags: IPLDirectSimulationFlags_IPL_DIRECTSIMULATIONFLAGS_OCCLUSION,
                    source: pos,
                    occlusionType: IPLOcclusionType_IPL_OCCLUSIONTYPE_RAYCAST,
                    ..Default::default()
                },
            );
        }
    }

    pub fn get_direct_outputs(&self, flags: SimulationFlags) -> IPLDirectEffectParams {
        let output = null_mut();
        unsafe {
            iplSourceGetOutputs(self.source, flags.convert(), output);

            let out = *output;
            return out.direct;
        }
    }
}

impl Drop for Source {
    fn drop(&mut self) {
        unsafe {
            iplSourceRelease(&mut self.source);
        }
    }
}
