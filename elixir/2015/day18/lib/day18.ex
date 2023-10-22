defmodule Day18 do
  @moduledoc """
  Documentation for `Day18`.
  """
  use Rustler, otp_app: :day18, crate: "lights"

  @input "input.txt"
  def part1() do
    grid = parse_grid_file(@input)

    Enum.reduce(1..100, grid, fn _, g ->
      step_grid(g)
    end)
    |> lights_on()
  end

  def part2() do
    grid = parse_grid_file(@input) |> stick_corners_on()

    Enum.reduce(1..100, grid, fn _, g ->
      step_grid(g) |> stick_corners_on()
    end)
    |> lights_on()
  end

  def parse_grid_file(path) do
    File.read!(path) |> parse_grid()
  end

  def how_many_lights_on?(grid) do
    lights_on(grid)
  end

  # When your NIF is loaded, it will override this function.
  def parse_grid(_str), do: :erlang.nif_error(:nif_not_loaded)
  def step_grid(_grid), do: :erlang.nif_error(:nif_not_loaded)
  def stick_corners_on(_grid), do: :erlang.nif_error(:nif_not_loaded)
  def lights_on(_grid), do: :erlang.nif_error(:nif_not_loaded)
  def output(_grid), do: :erlang.nif_error(:nif_not_loaded)
end
