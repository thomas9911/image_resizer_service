defmodule ImageResizer do
  @moduledoc """
  Documentation for `ImageResizer`.
  """

  def create_bucket(bucket) do
    bucket
    |> ExAws.S3.put_bucket(Application.fetch_env!(:image_resizer, :region))
    |> ExAws.request(ImageResizer.BinaryClient.ex_aws_config())
    |> then(fn
      {:ok, %{status_code: 200}} -> :ok
      {:error, {:http_error, 409, _}} -> :ok
      e -> e
    end)
  end

  defdelegate resize(bucket, input, output, width, height, method), to: ImageResizer.Client
end

# ImageResizer.resize("hallo", "falcon_4.jpg", "byex.jpg", 100, 100)
