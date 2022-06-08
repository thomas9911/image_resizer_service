import Config

config :image_resizer,
  shared_key: System.fetch_env!("IMAGE_RESIZER_SHARED_KEY"),
  address: System.fetch_env!("IMAGE_RESIZER_ADDRESS"),
  endpoint: System.fetch_env!("ASSET_ADDRESS"),
  region: System.fetch_env!("ASSET_REGION"),
  access_key: System.fetch_env!("ASSET_ACCESS_KEY"),
  secret_key: System.fetch_env!("ASSET_SECRET_KEY")
