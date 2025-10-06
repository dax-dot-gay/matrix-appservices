use bon::Builder;
use matrix_sdk::ruma::api::appservice as ruma_as;
use serde::{ Deserialize, Serialize };

/// A representation of the Namespace item [Matrix Specification](https://spec.matrix.org/latest/application-service-api/#definition-registration_namespace)
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Namespace {
    exclusive: bool,
    regex: String,
}

impl Namespace {
    /// Create a new [Namespace]
    ///
    /// # Arguments
    ///
    /// - `regex` (`impl Into<String>`) - Match regex
    /// - `exclusive` (`bool`) - Whether this namespace is exclusive
    ///
    /// # Returns
    ///
    /// - `Self` - Created [Namespace] object
    pub fn new(regex: impl Into<String>, exclusive: bool) -> Self {
        Self { exclusive, regex: regex.into() }
    }

    /// Whether this namespace is exclusive
    pub fn exclusive(&self) -> bool {
        self.exclusive
    }

    /// This namespace's regex
    pub fn regex(&self) -> String {
        self.regex.clone()
    }
}

impl From<String> for Namespace {
    fn from(value: String) -> Self {
        Self::new(value, false)
    }
}

impl From<&str> for Namespace {
    fn from(value: &str) -> Self {
        Self::new(value.to_string(), false)
    }
}

impl From<ruma_as::Namespace> for Namespace {
    fn from(value: ruma_as::Namespace) -> Self {
        Self::new(value.regex.clone(), value.exclusive)
    }
}

impl From<Namespace> for ruma_as::Namespace {
    fn from(value: Namespace) -> Self {
        Self::new(value.exclusive(), value.regex())
    }
}

/// Enum representing kinds of namespaces defined by the AppService specification
#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum NamespaceKind {
    /// A room alias that the application service is interested in
    Alias,

    /// A room ID that the application service is interested in
    Room,

    /// A user ID that the application service is interested in
    User,
}

impl From<String> for NamespaceKind {
    fn from(value: String) -> Self {
        Self::from(value.as_str())
    }
}

impl From<&str> for NamespaceKind {
    fn from(value: &str) -> Self {
        (
            match value.to_lowercase().as_str() {
                "alias" | "aliases" => Ok(Self::Alias),
                "room" | "rooms" => Ok(Self::Room),
                "user" | "users" => Ok(Self::User),
                other => Err(crate::Error::UnknownNamespaceKind(other.to_string())),
            }
        ).expect("Expected valid namespace kind")
    }
}

/// Collection of [Namespace] objects
#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct Namespaces {
    aliases: Vec<Namespace>,
    rooms: Vec<Namespace>,
    users: Vec<Namespace>,
}

impl Namespaces {
    /// Create a new namespace collection
    pub fn new() -> Self {
        Self::default()
    }

    /// Adds a new [Namespace] to the collection
    ///
    /// # Arguments
    ///
    /// - `kind` (`impl TryInto<NamespaceKind>`) - The kind of namespace (either an instance of the enum, or a compatible string representation)
    /// - `regex` (`impl Into<String>`) - Namespace regex
    /// - `exclusive` (`bool`) - Namespace exclusive
    pub fn add(
        &mut self,
        kind: impl Into<NamespaceKind>,
        regex: impl Into<String>,
        exclusive: bool
    ) {
        let kind: NamespaceKind = kind.into();
        match kind {
            NamespaceKind::Alias => self.aliases.push(Namespace::new(regex, exclusive)),
            NamespaceKind::Room => self.rooms.push(Namespace::new(regex, exclusive)),
            NamespaceKind::User => self.users.push(Namespace::new(regex, exclusive)),
        }
    }

    ///
    pub fn aliases(&self) -> Vec<Namespace> {
        self.aliases.clone()
    }

    ///
    pub fn rooms(&self) -> Vec<Namespace> {
        self.rooms.clone()
    }

    ///
    pub fn users(&self) -> Vec<Namespace> {
        self.users.clone()
    }
}

impl From<ruma_as::Namespaces> for Namespaces {
    fn from(value: ruma_as::Namespaces) -> Self {
        Self {
            aliases: value.aliases
                .clone()
                .into_iter()
                .map(|v| Namespace::from(v))
                .collect(),
            rooms: value.rooms
                .clone()
                .into_iter()
                .map(|v| Namespace::from(v))
                .collect(),
            users: value.users
                .clone()
                .into_iter()
                .map(|v| Namespace::from(v))
                .collect(),
        }
    }
}

impl From<Namespaces> for ruma_as::Namespaces {
    fn from(value: Namespaces) -> Self {
        let mut new = Self::new();
        new.aliases.extend(
            value
                .aliases()
                .into_iter()
                .map(|v| ruma_as::Namespace::from(v))
        );
        new.rooms.extend(
            value
                .rooms()
                .into_iter()
                .map(|v| ruma_as::Namespace::from(v))
        );
        new.users.extend(
            value
                .users()
                .into_iter()
                .map(|v| ruma_as::Namespace::from(v))
        );

        new
    }
}

/// Representation of the AppServiceRegistration config. For field values, reference the [Matrix Specification](https://spec.matrix.org/latest/application-service-api/#registration)
#[derive(Serialize, Deserialize, Clone, Debug, Builder)]
pub struct AppServiceRegistration {
    ///
    #[serde(rename = "id")]
    #[builder(start_fn, into)]
    pub application_id: String,

    ///
    #[builder(field)]
    pub namespaces: Namespaces,

    ///
    #[builder(field)]
    pub protocols: Vec<String>,

    ///
    #[serde(rename = "as_token")]
    #[builder(default = AppServiceRegistration::generate_key(24), into)]
    pub appservice_token: String,

    ///
    #[serde(rename = "hs_token")]
    #[builder(default = AppServiceRegistration::generate_key(24), into)]
    pub homeserver_token: String,

    ///
    #[builder(default = false)]
    pub rate_limited: bool,

    ///
    #[builder(default = false)]
    pub receive_ephemeral: bool,

    ///
    #[builder(into)]
    pub sender_localpart: String,

    ///
    #[builder(required, into)]
    pub url: Option<String>,
}

impl AppServiceRegistration {
    /// Generate a Base64-encoded key
    pub fn generate_key(length: usize) -> String {
        genrs_lib
            ::encode_key(genrs_lib::generate_key(length), genrs_lib::EncodingFormat::Base64)
            .expect("Encoding format should be supported!")
    }
}

impl<S: app_service_registration_builder::State> AppServiceRegistrationBuilder<S> {
    /// Add a single namespace to the registration
    pub fn namespace(
        mut self,
        kind: impl Into<NamespaceKind>,
        regex: impl Into<String>,
        exclusive: bool
    ) -> Self {
        self.namespaces.add(kind, regex, exclusive);
        self
    }

    /// Add several namespaces to the registration. The format is fairly forgiving -- `vec![("alias", "#room-name"), ...]` is valid.
    pub fn namespaces(
        mut self,
        namespaces: impl IntoIterator<Item = (impl Into<NamespaceKind>, impl Into<Namespace>)>
    ) -> Self {
        for (kind, namespace) in namespaces {
            let ns: Namespace = namespace.into();
            self = self.namespace(kind, ns.regex(), ns.exclusive());
        }

        self
    }

    /// Adds a single protocol to the registration
    pub fn protocol(mut self, protocol: impl Into<String>) -> Self {
        self.protocols.push(protocol.into());
        self
    }

    /// Adds several protocols to the registration
    pub fn protocols(mut self, protocols: impl IntoIterator<Item: Into<String>>) -> Self {
        for protocol in protocols {
            self.protocols.push(protocol.into());
        }
        self
    }
}

impl From<ruma_as::Registration> for AppServiceRegistration {
    fn from(value: ruma_as::Registration) -> Self {
        Self {
            application_id: value.id.clone(),
            namespaces: Namespaces::from(value.namespaces.clone()),
            protocols: value.protocols.clone().unwrap_or(vec![]),
            appservice_token: value.as_token.clone(),
            homeserver_token: value.hs_token.clone(),
            rate_limited: value.rate_limited.unwrap_or(false),
            receive_ephemeral: value.receive_ephemeral.clone(),
            sender_localpart: value.sender_localpart.clone(),
            url: value.url.clone(),
        }
    }
}

impl From<AppServiceRegistration> for ruma_as::Registration {
    fn from(value: AppServiceRegistration) -> Self {
        let init = ruma_as::RegistrationInit {
            id: value.application_id.clone(),
            url: value.url.clone(),
            as_token: value.appservice_token.clone(),
            hs_token: value.homeserver_token.clone(),
            sender_localpart: value.sender_localpart.clone(),
            namespaces: value.namespaces.clone().into(),
            rate_limited: Some(value.rate_limited),
            protocols: Some(value.protocols.clone()),
        };
        let mut reg = ruma_as::Registration::from(init);
        reg.receive_ephemeral = value.receive_ephemeral;
        reg
    }
}

impl AppServiceRegistration {
    /// Deserializes AppServiceRegistration from YAML
    pub fn from_yaml(yaml: impl AsRef<str>) -> crate::Result<Self> {
        Ok(serde_yml::from_str::<AppServiceRegistration>(yaml.as_ref())?)
    }

    /// Serializes AppServiceRegistration into YAML
    pub fn into_yaml(&self) -> crate::Result<String> {
        Ok(serde_yml::to_string(&self)?)
    }
}
