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
