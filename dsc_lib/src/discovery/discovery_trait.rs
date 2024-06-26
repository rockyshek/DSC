// Copyright (c) Microsoft Corporation.
// Licensed under the MIT License.

use crate::{dscresources::dscresource::DscResource, dscerror::DscError};
use std::collections::BTreeMap;

pub trait ResourceDiscovery {
    fn list_available_resources(&mut self, type_name_filter: &str, adapter_name_filter: &str) -> Result<BTreeMap<String, DscResource>, DscError>;
    fn discover_resources(&mut self, required_resource_types: &[String]) -> Result<BTreeMap<String, DscResource>, DscError>;
}
