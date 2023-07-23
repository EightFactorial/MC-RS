use enum_dispatch::enum_dispatch;
use json::JsonValue;
use strum::{Display, EnumIter, EnumString};

use crate::types::{ClassMap, Manifest, Version};

pub mod biome;
pub mod block;
pub mod diag;
pub mod entity;
pub mod info;
pub mod items;
pub mod lang;
pub mod packet;
pub mod particles;
pub mod recipe;
pub mod registry;
pub mod sound;
pub mod stats;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumIter, EnumString, Display)]
#[enum_dispatch(Dataset)]
#[strum(serialize_all = "lowercase")]
pub enum Datasets {
    Diagnostics(diag::Diagnostics),
    Info(info::Info),
    Armor(items::Armor),
}

/// The datasets that can be extracted
///
/// This trait is implemented for each dataset and is used to extract the data from the jar file
#[enum_dispatch]
pub trait Dataset {
    /// The minimum version this dataset is available in
    fn min(&self) -> &'static Option<Version>;

    /// The sets this dataset depends on
    fn deps(&self) -> &'static [Datasets];

    /// Parse the jar file and add the data to the json object
    fn parse(
        &self,
        version: &Version,
        manifest: &Manifest,
        classmap: &ClassMap,
        data: &mut JsonValue,
    );
}
