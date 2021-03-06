//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Copyright 2020 Joyent, Inc.
//
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_xml_rs;

pub const PG_NAME: &str = "property-group-name";
pub const PG_VALS: &str = "property-values";
pub const PG_NAME_PROTOCOL: &str = "protocol";
pub const PROP_NAME: &str = "property-name";
pub const PROP_VALUE: &str = "property-value";

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "nvpair")]
pub struct NvlistXmlArrayElement {
    #[serde(rename = "nvpair")]
    pub nvpairs: Option<Vec<NvpairXML>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "nvpair")]
pub struct NvpairXmlArrayElement {
    pub value: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "nvpair")]
pub struct NvpairXML {
    pub name: Option<String>,
    #[serde(rename = "type")]
    pub nvtype: Option<String>,
    pub value: Option<String>,
    #[serde(rename = "nvpair")]
    pub nvpair_elements: Option<Vec<NvpairXmlArrayElement>>,
    #[serde(rename = "nvlist")]
    pub nvlist_elements: Option<Vec<NvlistXmlArrayElement>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "edge")]
pub struct TopoEdgeXML {
    pub fmri: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "outgoing-edges")]
pub struct TopoOutgoingEdgesXML {
    #[serde(rename = "edge")]
    pub edges: Vec<TopoEdgeXML>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "vertex")]
pub struct TopoVertexXML {
    pub name: String,
    pub instance: String,
    pub fmri: String,
    #[serde(rename = "nvpair")]
    pub propgroups: Vec<NvpairXML>,
    #[serde(rename = "outgoing-edges")]
    pub outgoing_edges: Option<TopoOutgoingEdgesXML>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "nvpair")]
pub struct TopoVerticesXML {
    pub vertex: Vec<TopoVertexXML>,
}

//
// This is the top-level structure that should be used when deserializing the
// XML output produced by sastopo -x.
//
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "topo_digraph")]
pub struct TopoDigraphXML {
    #[serde(rename = "fmri-scheme")]
    pub scheme: String,
    pub nodename: String,
    #[serde(rename = "product-id")]
    pub product_id: String,
    #[serde(rename = "os-version")]
    pub os_version: String,
    pub timestamp: String,
    pub vertices: TopoVerticesXML,
}
