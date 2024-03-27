use crate::{repo::RepoTree, Args};
use color_eyre::eyre::Result;
use send_wrapper::SendWrapper;
use stardust_xr_fusion::{
    client::{Client, ClientState, FrameInfo, RootHandler},
    core::values::ResourceID,
    drawable::{Model, ModelPart},
    node::NodeType,
    spatial::{Spatial, Transform},
};

pub struct Kessler {
    root: Spatial,
    repo: SendWrapper<RepoTree>,
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
        let pot_root = pot_model.model_part("Root")?;
        Ok(Kessler {
            root: client.get_root().alias(),
            repo: SendWrapper::new(RepoTree::create(&pot_root, args)?),
            pot_model,
            pot_root,
        })
    }
}
impl RootHandler for Kessler {
    fn frame(&mut self, _info: FrameInfo) {}
    fn save_state(&mut self) -> ClientState {
        ClientState::from_root(&self.root)
    }
}
