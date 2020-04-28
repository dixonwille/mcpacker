use crate::app::*;
use crate::errors::Result;
use git2::{ObjectType, Repository, RepositoryState};
use semver::{Identifier, Version};
use std::{env, io};
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct BumpParams {
    #[structopt(flatten)]
    flags: BumpFlags,
    /// Set the version
    version: Option<Version>,
    /// Which field to bump [default: patch].
    #[structopt(subcommand)]
    field: Option<BumpField>,
}

#[derive(StructOpt, Debug)]
enum BumpField {
    /// Increment the major field and zero the others.
    Major {
        #[structopt(flatten)]
        flags: BumpFlags,
    },
    /// Increment the minor field and zero the patch and pre.
    Minor {
        #[structopt(flatten)]
        flags: BumpFlags,
    },
    /// Increment the patch field and zero the pre.
    Patch {
        #[structopt(flatten)]
        flags: BumpFlags,
    },
    /// Applies only pre and build flags to the new version. If neither flag is specified it clears them.
    #[structopt(alias = "meta")]
    Pre {
        #[structopt(flatten)]
        flags: BumpFlags,
    },
}

#[derive(StructOpt, Debug)]
struct BumpFlags {
    /// Creates a new commit and a git lightweight tag after applying the bump.
    ///
    /// Will commit anything that is staged. Will also force the commit to be the new commit if the tag already exists.
    /// Does not push to remote repository.
    #[structopt(short = "g", long = "git-tag")]
    git_tag: bool,
    /// Appends a pre-release version to the bump. Sould not be prepended with '-'.
    #[structopt(short = "p", long = "pre")]
    pre: Option<String>,
    /// Appends build metadata to the bump. Should not be prepended with '+'.
    #[structopt(short = "m", long = "meta")]
    meta: Option<String>,
}

impl BumpParams {
    pub fn run(&self) -> Result<()> {
        let mut manifest = get_manifest()?;
        let mut git = self.flags.git_tag;
        match self.version.as_ref() {
            Some(ver) => manifest.version = ver.clone(),
            None => match self.field.as_ref() {
                Some(f) => match f {
                    BumpField::Major { flags } => {
                        manifest.version.increment_major();
                        apply_ver_flags(&mut manifest, flags);
                        git = flags.git_tag;
                    }
                    BumpField::Minor { flags } => {
                        manifest.version.increment_minor();
                        apply_ver_flags(&mut manifest, flags);
                        git = flags.git_tag;
                    }
                    BumpField::Patch { flags } => {
                        manifest.version.increment_patch();
                        apply_ver_flags(&mut manifest, flags);
                        git = flags.git_tag;
                    }
                    BumpField::Pre { flags } => {
                        apply_ver_flags(&mut manifest, flags);
                        git = flags.git_tag;
                    }
                },
                None => {
                    manifest.version.increment_patch();
                    apply_ver_flags(&mut manifest, &self.flags);
                }
            },
        }
        manifest.to_writer(create_manifest_file()?)?;
        if git {
            let repo = Repository::open(env::current_dir()?)?;
            // Make sure the repo is in a clean state (not in the middle of rebase or merge)
            if repo.state() != RepositoryState::Clean {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "repository is not in a clean state",
                )
                .into());
            }
            // Stage the manifest file on top of any currently staged objects.
            let mut idx = repo.index()?;
            let manifest_path = Path::new(MANIFEST_FILE);
            idx.add_path(manifest_path)?;
            let tree = idx.write_tree()?;
            idx.write()?;
            // Commit the staged objects
            let tree = repo.find_tree(tree)?;
            let sig = repo.signature()?;
            let head = repo.head()?.peel_to_commit()?;
            let commit = repo.commit(
                Some("HEAD"),
                &sig,
                &sig,
                &format!("update to version {}", manifest.version),
                &tree,
                &[&head],
            )?;
            let commit_object = repo.find_object(commit, Some(ObjectType::Commit))?;
            // Tag the commit that was just created
            let _ = repo.tag_lightweight(&manifest.version.to_string(), &commit_object, true)?;
        }
        Ok(())
    }
}

fn apply_ver_flags(manifest: &mut Manifest, flags: &BumpFlags) {
    match flags.pre.as_ref() {
        Some(pre) => manifest.version.pre = flag_to_identifiers(pre),
        None => manifest.version.pre = Vec::new(),
    }
    match flags.meta.as_ref() {
        Some(meta) => manifest.version.build = flag_to_identifiers(meta),
        None => manifest.version.build = Vec::new(),
    }
}

fn flag_to_identifiers(flag: impl AsRef<str>) -> Vec<Identifier> {
    let mut ids = Vec::new();
    for id in flag.as_ref().split('.') {
        let i = match id.parse::<u64>() {
            Ok(i) => Identifier::Numeric(i),
            Err(_) => Identifier::AlphaNumeric(id.to_string()),
        };
        ids.push(i);
    }
    ids
}