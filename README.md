# Image Resizer Service

Service that loads image from Aws s3, resizes it and uploads it.

The service can be called using gRPC and resizes an image with the [image](https://github.com/image-rs/image) crate.

Image formats that `image` crate supports:

- PNG
- JPEG
- GIF
- BMP
- ICO
- PNM
- farbfeld

Easiest is to run with docker-compose:

```sh
docker-compose up -d
```

Contains two clients:

- Elixir client

- Python client
  - Has a Gui
