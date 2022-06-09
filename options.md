"resize_to_fit"
"resize_to_fill"
"resize_to_limit"
"resize_and_pad"



```elixir
  defp resize_file(%Mogrify.Image{} = file, :resize_to_fit, size) do
    Mogrify.resize(file, size)
  end

  defp resize_file(%Mogrify.Image{} = file, :resize_to_fill, size) do
    file
    |> Mogrify.gravity("Center")
    |> Mogrify.resize_to_fill(size)
  end

  defp resize_file(%Mogrify.Image{} = file, :resize_to_limit, size), do: Mogrify.resize_to_limit(file, size)

  defp resize_file(%Mogrify.Image{} = file, :resize_and_pad, size) do
    file
    |> Mogrify.resize(size)
    |> Mogrify.extent(size)
    |> Mogrify.gravity("Center")
  end
```
