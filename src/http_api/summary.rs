use serde::{Deserialize, Serialize};

fn default_vhost_key() -> String {
    String::new()
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Summary {
    pub urls: Box<Urls>,
    pub tests: Tests,
}

#[derive(Serialize, Deserialize, Debug)]
#[allow(clippy::pub_underscore_fields)]
pub struct Tests {
    pub requests: String,
    pub errors: String,
    pub redirects: String,
    #[serde(rename = "[vhost]", default = "default_vhost_key")]
    pub vhost: String,
    #[serde(default = "default_vhost_key")]
    pub _vhost: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Urls {
    pub versions: String,
    pub summaries: String,
    pub rusages: String,
    pub self_proc_stats: String,
    pub system_proc_stats: String,
    pub meminfos: String,
    pub authors: String,
    pub features: String,
    pub requests: String,
    pub vhosts: String,
    pub streams: String,
    pub clients: String,
    pub raw: String,
    pub clusters: String,
    pub perf: String,
    pub tcmalloc: String,
}
