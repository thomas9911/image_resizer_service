defmodule ImageResizer.BinaryClient do
  def connect do
    GRPC.Stub.connect(address())
  end

  # ImageResizer.BinaryClient.resize("hallo", "falcon_4.jpg", "plke.jpg", 250, 250)

  def resize(bucket, input, output, width, height, method \\ :fill) do
    with {:ok, %{body: body, headers: headers, status_code: 200}} <-
           bucket |> ExAws.S3.get_object(input) |> ExAws.request(ex_aws_config()),
         {_, format} <- :lists.keyfind("Content-Type", 1, headers),
         {:ok, reply} <- inner_resize(body, format, width, height, method),
         %Resizer.ResizeBinaryReply{image: image, message: "success", status: :OK} <- reply do
      bucket
      |> ExAws.S3.put_object(output, image, content_type: format)
      |> ExAws.request(ex_aws_config())
    end
  end

  defp inner_resize(binary, format, width, height, method) do
    request =
      make_request(
        binary,
        format,
        width,
        height,
        method
      )

    case connect() do
      {:ok, channel} -> Resizer.ResizerBinary.Stub.resize_binary(channel, request)
      e -> e
    end
  end

  def make_request(binary, format, width, height, method) do
    Resizer.ResizeBinaryRequest.new(
      image: binary,
      format: format,
      width: width,
      height: height,
      method: to_method(method)
    )
  end

  def address do
    Application.get_env(:image_resizer, :binary_address, "server:50052")
  end

  # def config do
  #   %{
  #     endpoint: Application.fetch_env!(:image_resizer, :endpoint),
  #     region: Application.fetch_env!(:image_resizer, :region),
  #     access_key: Application.fetch_env!(:image_resizer, :access_key),
  #     secret_access_key: Application.fetch_env!(:image_resizer, :secret_key)
  #   }
  # end

  def ex_aws_config do
    %{
      host: host,
      port: port,
      scheme: scheme
    } = Application.fetch_env!(:image_resizer, :endpoint) |> URI.parse()

    [
      host: host,
      port: port,
      scheme: scheme,
      region: Application.fetch_env!(:image_resizer, :region),
      access_key_id: Application.fetch_env!(:image_resizer, :access_key),
      secret_access_key: Application.fetch_env!(:image_resizer, :secret_key)
    ]
  end

  defp to_method(:fill), do: :FILL
  defp to_method(:fit), do: :FIT
  defp to_method(:limit), do: :LIMIT
  defp to_method(:pad), do: :PAD
  defp to_method(method), do: method
end
