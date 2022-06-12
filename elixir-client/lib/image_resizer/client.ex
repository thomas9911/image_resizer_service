defmodule ImageResizer.Client do
  def connect do
    GRPC.Stub.connect(address())
  end

  def resize(bucket, input, output, width, height) do
    request =
      make_request(
        bucket,
        input,
        output,
        width,
        height
      )

    case connect() do
      {:ok, channel} -> Resizer.Resizer.Stub.resize(channel, request)
      e -> e
    end
  end

  def make_request(bucket, input, output, width, height) do
    config = make_config()

    Resizer.ResizeRequest.new(
      bucket: bucket,
      input: input,
      output: output,
      width: width,
      height: height,
      config: config
    )
  end

  def make_config(config \\ config()) do
    iv = :crypto.strong_rand_bytes(12)

    plaintext =
      config
      |> Map.put(:nonce, Base.encode16(:crypto.strong_rand_bytes(16)))
      |> Jason.encode!()

    {text, tag} =
      :crypto.crypto_one_time_aead(:aes_256_gcm, shared_key(), iv, plaintext, <<>>, true)

    :erlang.iolist_to_binary([iv, text, tag])
  end

  def address do
    Application.get_env(:image_resizer, :address, "server:50051")
  end

  def shared_key do
    Application.fetch_env!(:image_resizer, :shared_key)
    |> Base.decode64!()
  end

  def config do
    %{
      endpoint: Application.fetch_env!(:image_resizer, :endpoint),
      region: Application.fetch_env!(:image_resizer, :region),
      access_key: Application.fetch_env!(:image_resizer, :access_key),
      secret_access_key: Application.fetch_env!(:image_resizer, :secret_key)
    }
  end
end
