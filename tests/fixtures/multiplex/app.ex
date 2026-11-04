defmodule MultiplexFixture.App do
  @moduledoc """
  Sanitized excerpt of `ggen_igniter/lib/ggen_igniter/application.ex` (ggen_igniter
  repo). Fixture for the four-language compositor multiplex witness (R2 lane).
  A deliberate syntax violation is appended so elixir-ls relays a REAL parse
  diagnostic through the compositor.
  """

  use Application

  @doc """
  Shape mirrors the real application module: a supervised Finch pool started
  conditionally, mirroring the real `start/2`.
  """
  @impl true
  def start(_type, _args) do
    finch_children =
      if Code.ensure_loaded?(Finch) do
        [{Finch, name: MultiplexFixture.Finch}]
      else
        []
      end

    children = finch_children

    opts = [strategy: :one_for_one, name: MultiplexFixture.Supervisor]
    Supervisor.start_link(children, opts)
  end

  # deliberate violation: unclosed `do` block → real elixir-ls syntax diagnostic
  def deliberate_violation do
    {:error, "unclosed block"
end
