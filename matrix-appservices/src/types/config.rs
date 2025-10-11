use std::{ fmt::Debug, net::{ IpAddr, SocketAddr } };
use serde::{ de::DeserializeOwned, Deserialize, Serialize };

use crate::types::{ registration::{ Namespace, NamespaceKind }, AppServiceRegistration };

/// Version of [`crate::types::registration::Namespace`] for [`AppServiceConfig`]
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfigNamespace {
    /// Namespace type (alias, room, user)
    pub kind: NamespaceKind,

    /// Namespace regex
    pub regex: String,

    /// Whether this namespace is exclusive
    #[serde(default)]
    pub exclusive: bool,
}

/// Local bind configuration for ease of config syntax
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BindConfig {
    /// Bind to `host:port`
    Url(SocketAddr),

    /// Bind to (`host`, `port`)
    Parts(IpAddr, u16),
}

impl From<BindConfig> for SocketAddr {
    fn from(value: BindConfig) -> Self {
        match value {
            BindConfig::Url(socket_addr) => socket_addr,
            BindConfig::Parts(ip_addr, port) => SocketAddr::new(ip_addr, port),
        }
    }
}

/// High-level appservice configuration, with local bind config and support for extra fields
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppServiceConfig<E: Clone + Debug + Serialize = ()> {
    /// A unique, user-defined ID of the application service which will never change.
    pub application_id: String,

    /// The namespaces that the application service is interested in.
    #[serde(default)]
    pub namespaces: Vec<ConfigNamespace>,

    /// The external protocols which the application service provides (e.g. IRC).
    #[serde(default)]
    pub protocols: Vec<String>,

    /// A secret token that the application service will use to authenticate requests to the homeserver.
    #[serde(alias = "as_token")]
    pub appservice_token: String,

    /// A secret token that the homeserver will use authenticate requests to the application service.
    #[serde(alias = "hs_token")]
    pub homeserver_token: String,

    /// Whether requests from masqueraded users are rate-limited. The sender is excluded. Defaults to `false`
    #[serde(default)]
    pub rate_limited: bool,

    /// Whether the application service wants to receive ephemeral data. Defaults to `false`
    #[serde(default)]
    pub receive_ephemeral: bool,

    /// The localpart of the user associated with the application service. Events will be sent to the AS if this user is the target of the event, or is a joined member of the room where the event occurred.
    #[serde(alias = "sender", alias = "sender_localpart")]
    pub username: String,

    /// The URL for the application service. May include a path after the domain name.
    #[serde(alias = "url")]
    pub service_url: String,

    /// The location to bind the appservice server to
    pub bind_address: BindConfig,

    /// Extra options. User-defined
    #[serde(default = "Option::default", flatten)]
    pub extra: Option<E>,
}

impl<E: Clone + Debug + Serialize + DeserializeOwned> AppServiceConfig<E> {
    /// Generates a random token
    pub fn generate_key(length: usize) -> String {
        AppServiceRegistration::generate_key(length)
    }

    /// Loads a config from YAML
    pub fn from_yaml(yaml: impl Into<String>) -> crate::Result<Self> {
        Ok(serde_norway::from_str::<Self>(Into::<String>::into(yaml).as_str())?)
    }

    /// Saves config to YAML
    pub fn into_yaml(&self) -> crate::Result<String> {
        Ok(serde_norway::to_string(self)?)
    }

    /// Generates an [`AppServiceRegistration`] from this config
    pub fn registration(&self) -> AppServiceRegistration {
        AppServiceRegistration::builder(self.application_id.clone())
            .protocols(self.protocols.clone())
            .appservice_token(self.appservice_token.clone())
            .homeserver_token(self.homeserver_token.clone())
            .rate_limited(self.rate_limited)
            .receive_ephemeral(self.receive_ephemeral)
            .sender_localpart(self.username.clone())
            .url(self.service_url.clone())
            .namespaces(
                self.namespaces
                    .clone()
                    .into_iter()
                    .map(|ConfigNamespace { kind, regex, exclusive }| (
                        kind,
                        Namespace::new(regex, exclusive),
                    ))
            )
            .build()
    }
}
