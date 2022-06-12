defmodule ImageResizer.MixProject do
  use Mix.Project

  def project do
    [
      app: :image_resizer,
      version: "0.1.0",
      elixir: "~> 1.13",
      start_permanent: Mix.env() == :prod,
      deps: deps(),
      elixirc_paths: ["lib", "priv/proto"]
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger]
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:grpc, github: "taxjar/grpc", branch: "db-handle-otp-23-24"},
      {:cowlib, "~> 2.9.0", override: true},
      {:jason, ">= 0.0.0"},

      # s3
      {:ex_aws, "~> 2.0"},
      {:ex_aws_s3, "~> 2.0"},
      {:poison, "~> 3.0"},
      {:hackney, "~> 1.9"},
      {:sweet_xml, "~> 0.6.6"}
    ]
  end
end
