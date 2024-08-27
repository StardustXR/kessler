use crate::Args;
use color_eyre::eyre::{bail, Result};
use git2::{Oid, Repository};
use glam::{vec3a, Vec3A};
use ouroboros::self_referencing;
use rand::Rng;
use rustc_hash::FxHashMap;
use stardust_xr_fusion::{
    core::values::ResourceID,
    drawable::Model,
    root::FrameInfo,
    spatial::{Spatial, SpatialAspect, Transform},
};

fn make_graph<'repo>(
    repo: &'repo Repository,
    spatial: &impl SpatialAspect,
) -> Result<FxHashMap<Oid, Commit<'repo>>, git2::Error> {
    let mut revwalk = repo.revwalk()?;
    for (branch, _) in repo.branches(None)?.filter_map(Result::ok) {
        if let Ok(c) = branch.get().peel_to_commit() {
            // basically we just wanna get ALL the commits so push all the branch heads to the walk
            revwalk.push(c.id())?;
        }
    }

    // now walk the whole graph, get their commits if they exist, and put them in the wrapper
    Ok(revwalk
        .filter_map(Result::ok)
        .filter_map(|m| repo.find_commit(m).ok())
        .map(|c| (c.id(), Commit::from_raw(&c, spatial)))
        .collect())
}

#[self_referencing]
pub struct RepoTree {
    git: Repository,

    #[borrows(git)]
    #[not_covariant]
    commit_graph: FxHashMap<Oid, Commit<'this>>,
}
impl RepoTree {
    pub fn create(origin: &impl SpatialAspect, args: Args) -> Result<RepoTree> {
        let git = Repository::open(args.repo_path.canonicalize().unwrap())?;
        if git.is_bare() {
            bail!("git repo is bare, oopsies");
        }
        if git.is_empty()? {
            bail!("git repo is empty, oopsies");
        }
        Ok(RepoTree::try_new(git, |repo| {
            dbg!(make_graph(repo, origin))
        })?)
    }
    pub fn update(&mut self, frame_info: &FrameInfo) {
        // let ideal_length = self.with_commit_graph(|c| c.len() as f32).sqrt();
        // self.with_commit_graph_mut(|commits| {
        //     for commit in commits.values_mut() {
        //         let mut force = vec3a(0.0, 0.0, 0.0);
        //         for parent in commit.raw.parents() {
        //             let Some(parent) = commits.get_mut(&parent.id()) else {
        //                 continue;
        //             };
        //             let d = parent.pos - commit.pos;
        //             let distance = d.length();
        //             let force_magnitude = (ideal_length - distance) / distance;

        //             force += d * force_magnitude;
        //         }
        //         for other_commit in commits.values_mut() {
        //             if commit.raw.id() == other_commit.raw.id() {
        //                 continue;
        //             }
        //             if commit
        //                 .raw
        //                 .parents()
        //                 .map(|p| p.id())
        //                 .find(|p| p == &commit.raw.id())
        //                 .is_some()
        //             {
        //                 continue;
        //             }

        //             let d = other_commit.pos - commit.pos;
        //             let distance = d.length();
        //             let force_magnitude = 1.0 / distance;

        //             force += force_magnitude * d / distance;
        //         }
        //         commit.pos += force;
        //         if force.length_squared() > f32::EPSILON {
        //             commit
        //                 .root
        //                 .set_local_transform(Transform::from_translation(commit.pos))
        //                 .unwrap();
        //         }
        //     }
        // });
    }
}

pub struct Commit<'repo> {
    raw: git2::Commit<'repo>,
    root: Spatial,
    pos: Vec3A,
    model: Model,
}
impl<'repo> std::fmt::Debug for Commit<'repo> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.raw.summary().fmt(f)
    }
}
impl<'repo> Commit<'repo> {
    fn from_raw(commit: &git2::Commit<'repo>, spatial: &impl SpatialAspect) -> Self {
        let root = Spatial::create(spatial, Transform::identity(), false).unwrap();
        Commit {
            raw: commit.clone(),
            // initialize the position randomly for the force directed graph
            pos: vec3a(
                rand::thread_rng().gen(),
                rand::thread_rng().gen::<f32>().abs(), // and we gotta make sure it's a positive value
                rand::thread_rng().gen(),
            ),
            model: Model::create(
                &root,
                Transform::identity(),
                &ResourceID::new_namespaced("kessler", "pot"),
            )
            .unwrap(),
            root,
        }
    }
}
