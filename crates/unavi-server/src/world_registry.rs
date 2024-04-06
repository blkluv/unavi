use std::{future::Future, sync::Arc};

use axum::{routing::get, Json, Router};
use didkit::{
    ssi::did::{RelativeDIDURL, VerificationMethod, VerificationMethodMap},
    Document, JWK,
};
use dwn::{
    actor::{Actor, VerifiableCredential},
    message::descriptor::{
        protocols::{ProtocolDefinition, ProtocolsFilter},
        records::Version,
    },
    store::{DataStore, MessageStore},
    DWN,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tracing::{info, warn};

const IDENTITY_PATH: &str = ".unavi/registry_identity.json";
const KEY_FRAGMENT: &str = "key-0";
const PROTOCOL_DEFINITION: &str =
    include_str!("../../../wired-protocol/social/dwn/protocols/world-registry.json");
const PROTOCOL_VERSION: &str = "0.0.1";

pub async fn router(
    dwn: Arc<DWN<impl DataStore, impl MessageStore>>,
    domain: &str,
) -> (Router, impl Future) {
    let domain = domain.replace(':', "%3A");
    let did = format!("did:web:{}", domain);

    let actor = if let Ok(identity) = std::fs::read_to_string(IDENTITY_PATH) {
        let identity: RegistryIdentity =
            serde_json::from_str(&identity).expect("Failed to parse registry identity");

        if identity.did == did {
            Actor {
                attestation: identity.vc_key.clone().into(),
                authorization: identity.vc_key.into(),
                did: identity.did,
                dwn: dwn.clone(),
                remotes: Vec::new(),
            }
        } else {
            warn!("Registry DID mismatch. Overwriting identity file.");
            std::fs::remove_file(IDENTITY_PATH).unwrap();
            create_identity(did, dwn)
        }
    } else {
        create_identity(did, dwn)
    };

    info!("Registry DID: {}", actor.did);

    let mut document = Document::new(&actor.did);

    document.verification_method = Some(vec![VerificationMethod::Map(VerificationMethodMap {
        controller: actor.did.clone(),
        id: format!("{}#{}", &actor.did, KEY_FRAGMENT),
        public_key_jwk: Some(actor.authorization.jwk.clone().to_public()),
        type_: "JsonWebKey2020".to_string(),
        ..Default::default()
    })]);

    document.assertion_method = Some(vec![VerificationMethod::RelativeDIDURL(RelativeDIDURL {
        fragment: Some(KEY_FRAGMENT.to_string()),
        ..Default::default()
    })]);

    document.authentication = Some(vec![VerificationMethod::RelativeDIDURL(RelativeDIDURL {
        fragment: Some(KEY_FRAGMENT.to_string()),
        ..Default::default()
    })]);

    let document = Arc::new(document);
    let router = Router::new().route(
        "/.well-known/did.json",
        get(|| async move { Json(document.clone()) }),
    );

    let create_registry = async move {
        let value: Value = serde_json::from_str(PROTOCOL_DEFINITION).unwrap();
        let protocol = value["protocol"].as_str().unwrap().to_string();
        let version = Version::parse(PROTOCOL_VERSION).unwrap();

        let query = actor
            .query_protocols(ProtocolsFilter {
                protocol,
                versions: vec![version.clone()],
            })
            .process()
            .await
            .unwrap();

        if query.entries.is_empty() {
            info!("Creating world registry v{}", PROTOCOL_VERSION);
            let definition = json_to_defition(value);
            actor
                .register_protocol(definition)
                .protocol_version(version)
                .process()
                .await
                .unwrap();
        }
    };

    (router, create_registry)
}

fn create_identity<D, M>(did: String, dwn: Arc<DWN<D, M>>) -> Actor<D, M>
where
    D: DataStore,
    M: MessageStore,
{
    // Create a did:key and convert it to a did:web.
    let mut actor = Actor::new_did_key(dwn).unwrap();
    let key_id = format!("{}#{}", did, KEY_FRAGMENT);
    actor.attestation.key_id = key_id.clone();
    actor.authorization.key_id = key_id;
    actor.did = did;

    let identity = RegistryIdentity {
        did: actor.did.clone(),
        vc_key: actor.authorization.clone().into(),
    };
    let identity = serde_json::to_string(&identity).unwrap();

    std::fs::write(IDENTITY_PATH, identity).unwrap();

    actor
}

fn json_to_defition(json: Value) -> ProtocolDefinition {
    let protocol = json["protocol"].as_str().unwrap().to_string();

    let structure = json["structure"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| (k.to_string(), serde_json::from_value(v.clone()).unwrap()))
        .collect();

    let types = json["types"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| (k.to_string(), serde_json::from_value(v.clone()).unwrap()))
        .collect();

    ProtocolDefinition {
        protocol,
        published: true,
        structure,
        types,
    }
}

#[derive(Clone, Deserialize, Serialize)]
struct VcKey {
    jwk: JWK,
    key_id: String,
}

impl From<VerifiableCredential> for VcKey {
    fn from(vc: VerifiableCredential) -> Self {
        Self {
            jwk: vc.jwk,
            key_id: vc.key_id,
        }
    }
}

impl From<VcKey> for VerifiableCredential {
    fn from(vc: VcKey) -> Self {
        Self {
            jwk: vc.jwk,
            key_id: vc.key_id,
        }
    }
}

#[derive(Deserialize, Serialize)]
struct RegistryIdentity {
    did: String,
    vc_key: VcKey,
}
