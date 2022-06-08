FROM elixir:latest

WORKDIR /app
COPY lib lib
COPY priv priv
COPY config config
COPY mix.exs mix.lock ./
RUN mix local.hex --force
RUN mix local.rebar --force
RUN mix deps.get
RUN mix compile
CMD iex -S mix run
