use crate::Args;
use color_eyre::eyre::Result;
use send_wrapper::SendWrapper;
use stardust_xr_fusion::{
    client::Client,
    core::values::ResourceID,
    drawable::{Model, ModelPart},
    node::{MethodResult, NodeType},
    root::{ClientState, FrameInfo, RootHandler},
    spatial::{Spatial, Transform},
};

pub struct Kessler {
    // repo: SendWrapper<RepoTree>,
    pot_model: Model,
    pot_root: ModelPart,
}
impl Kessler {
    pub fn new(client: &Client, args: Args) -> Result<Self> {
        let pot_model = Model::create(
            client.get_root(),
            Transform::identity(),
            &ResourceID::new_namespaced("kessler", "pot"),
        )?;
        let pot_root = pot_model.part("Root")?;
        Ok(Kessler {
            // repo: SendWrapper::new(RepoTree::create(&pot_root, args)?),
            pot_model,
            pot_root,
        })
    }
}
impl RootHandler for Kessler {
    fn frame(&mut self, _info: FrameInfo) {}
    fn save_state(&mut self) -> MethodResult<ClientState> {
        Ok(ClientState::default())
    }
}
