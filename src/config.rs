use aes_gcm::aead::NewAead;
use aes_gcm::{Aes256Gcm, Key};
use derivative::Derivative;
use once_cell::sync::Lazy;
use zeroize::{Zeroize, ZeroizeOnDrop};

static CONFIG: Lazy<ResizerConfig> = Lazy::new(ResizerConfig::init_config);

#[derive(Derivative, Zeroize, ZeroizeOnDrop)]
#[derivative(Default)]
/// container for global config and wrapper around `once_cell`
pub struct ResizerConfig {
    #[derivative(Default(value = "\"[::1]:50051\".to_string()"))]
    address: String,
    #[derivative(Default(value = "4000"))]
    max_size: u32,
    shared_key: Option<String>,
}

impl ResizerConfig {
    /// initialize global config and panics on missing required configurations
    pub fn init() {
        Lazy::force(&CONFIG);
        Self::fetch_and_validate_shared_key();
    }

    fn init_config() -> ResizerConfig {
        let mut config = ResizerConfig::default();
        config.from_env();
        config
    }

    /// Gather info from env variables
    pub fn from_env(&mut self) {
        if let Ok(var) = std::env::var("IMAGE_RESIZER_ADDRESS") {
            self.address = var;
        };
        if let Ok(var) = std::env::var("IMAGE_RESIZER_MAX_SIZE") {
            if let Ok(var) = var.parse() {
                self.max_size = var;
            }
        };

        if let Ok(var) = std::env::var("IMAGE_RESIZER_SHARED_KEY") {
            self.shared_key = Some(var);
        };
    }

    ///
    /// # Errors
    ///
    /// Returns error when address is not a valid address
    ///
    pub fn address(
    ) -> Result<std::net::SocketAddr, <std::net::SocketAddr as std::str::FromStr>::Err> {
        CONFIG.address.parse()
    }

    #[must_use]
    pub fn max_size() -> u32 {
        CONFIG.max_size
    }

    ///
    /// # Panics
    ///
    /// panics on unset shared key,
    /// if it is not base64 or if it is of invalid length, other than 32 bytes
    ///
    #[must_use]
    pub fn shared_key() -> Key<<Aes256Gcm as NewAead>::KeySize> {
        let key = Self::fetch_and_validate_shared_key();
        *Key::from_slice(&key)
    }

    fn fetch_and_validate_shared_key() -> Vec<u8> {
        let key = CONFIG.shared_key.as_ref().expect("shared key not set");
        let key = base64::decode(key).expect("shared key invalid base64");
        if key.len() != 32 {
            panic!("shared key invalid length")
        };
        key
    }

    #[must_use]
    pub fn generate_shared_key() -> String {
        use rand::RngCore;

        let mut key: [u8; 32] = [0; 32];
        let mut generator = rand::thread_rng();

        generator.fill_bytes(&mut key);

        base64::encode(key)
    }
}
