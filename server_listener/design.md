# design

## flow

- Client:

  - fetch image from assets
  - put image in redis under 'key' with type binary
  - put resize options in redis under 'other key' with type hashmap
  - publish message to 'channel'

- Server:

  - Listen to channel 'channel'
  - on message fetch resize options from redis
  - fetch image from redis
  - resize images based on options
  - put images back into redis under different keys
  - publish message to channel 'other channel' with the different keys

- Client:

  - List to channel 'other channel'
  - fetch images from redis
  - store images in assets
  - delete keys 'key' and different keys
