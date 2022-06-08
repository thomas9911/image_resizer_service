defmodule ImageResizer do
  @moduledoc """
  Documentation for `ImageResizer`.
  """

  defdelegate resize(bucket, input, output, width, height), to: ImageResizer.Client
end

# ImageResizer.resize("hallo", "falcon_4.jpg", "byex.jpg", 100, 100)
