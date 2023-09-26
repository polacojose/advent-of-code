defmodule AdventOfCode.Year2016.Day06.Part01 do
  @input "input.txt"

  def parse() do
    File.stream!(@input)
    |> Enum.map(&String.trim/1)
    |> Enum.map(&String.split/1)
    |> Enum.map(&format_input/1)
  end

  def how_many_lights_light?() do
    Enum.reduce(parse(), init_grid(false), fn instruction, grid ->
      case instruction do
        {:turn_on, s, e} -> gen_coordinates(s, e) |> turn_on(grid)
        {:turn_off, s, e} -> gen_coordinates(s, e) |> turn_off(grid)
        {:toggle, s, e} -> gen_coordinates(s, e) |> toggle(grid)
      end
    end)
    |> Enum.count(fn {_, v} -> v == true end)
  end

  def turn_on(c, m) do
    Enum.reduce(c, m, fn k, m -> %{m | k => true} end)
  end

  def turn_off(c, m) do
    Enum.reduce(c, m, fn k, m -> %{m | k => false} end)
  end

  def toggle(c, m) do
    Enum.reduce(c, m, fn k, m ->
      %{m | k => !m[k]}
    end)
  end

  def gen_coordinates({x, y}, {xx, yy}) do
    for x <- x..xx, y <- y..yy, do: {x, y}
  end

  def init_grid(value) do
    for(
      x <- 0..999,
      y <- 0..999,
      do: {x, y}
    )
    |> Enum.reduce(%{}, fn key, map -> Map.put(map, key, value) end)

    # %{map | key => value} end)
  end

  def format_input([_, "on", c, _, d]), do: to_format(:turn_on, c, d)
  def format_input(["toggle", c, _, d]), do: to_format(:toggle, c, d)
  def format_input([_, "off", c, _, d]), do: to_format(:turn_off, c, d)

  def to_format(type, c, d) do
    [x, y] = String.split(c, ",") |> Enum.map(&String.to_integer/1)
    [xx, yy] = String.split(d, ",") |> Enum.map(&String.to_integer/1)
    {type, {x, y}, {xx, yy}}
  end
end

defmodule AdventOfCode.Year2016.Day06.Part02 do
  @input "input.txt"

  def parse() do
    File.stream!(@input)
    |> Enum.map(&String.trim/1)
    |> Enum.map(&String.split/1)
    |> Enum.map(&format_input/1)
  end

  def how_many_lights_light?() do
    Enum.reduce(parse(), init_grid(0), fn instruction, grid ->
      case instruction do
        {:turn_on, s, e} -> gen_coordinates(s, e) |> turn_on(grid)
        {:turn_off, s, e} -> gen_coordinates(s, e) |> turn_off(grid)
        {:toggle, s, e} -> gen_coordinates(s, e) |> toggle(grid)
      end
    end)
    |> Enum.map(fn {_, v} -> v end)
    |> Enum.sum()
  end

  def turn_on(c, m) do
    Enum.reduce(c, m, fn k, m -> %{m | k => m[k] + 1} end)
  end

  def turn_off(c, m) do
    Enum.reduce(c, m, fn k, m -> %{m | k => max(m[k] - 1, 0)} end)
  end

  def toggle(c, m) do
    Enum.reduce(c, m, fn k, m ->
      %{m | k => m[k] + 2}
    end)
  end

  def gen_coordinates({x, y}, {xx, yy}) do
    for x <- x..xx, y <- y..yy, do: {x, y}
  end

  def init_grid(value) do
    for(
      x <- 0..999,
      y <- 0..999,
      do: {x, y}
    )
    |> Enum.reduce(%{}, fn key, map -> Map.put(map, key, value) end)

    # %{map | key => value} end)
  end

  def format_input([_, "on", c, _, d]), do: to_format(:turn_on, c, d)
  def format_input(["toggle", c, _, d]), do: to_format(:toggle, c, d)
  def format_input([_, "off", c, _, d]), do: to_format(:turn_off, c, d)

  def to_format(type, c, d) do
    [x, y] = String.split(c, ",") |> Enum.map(&String.to_integer/1)
    [xx, yy] = String.split(d, ",") |> Enum.map(&String.to_integer/1)
    {type, {x, y}, {xx, yy}}
  end
end

IO.puts("Part01")
AdventOfCode.Year2016.Day06.Part01.how_many_lights_light?() |> IO.puts()
IO.puts("Part02")
AdventOfCode.Year2016.Day06.Part02.how_many_lights_light?() |> IO.puts()
