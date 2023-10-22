defmodule Day18 do
  @moduledoc """
  Documentation for `Day18`.
  """

  @input "input.txt"
  def part1(steps) do
    grid = parse(@input)

    if steps >= 1 do
      Enum.reduce(0..(steps - 1), grid, fn _, grid ->
        step(grid)
      end)
    else
      grid
    end
  end

  def lights_on?(grid) do
    for(y <- grid, {_, true} <- y, do: true) |> Enum.count()
  end

  def step(grid) do
    Enum.map(grid, fn l ->
      Enum.map(l, &new_mode(grid, &1))
    end)
  end

  def new_mode(grid, {vec, on}) do
    num_neigbors = neighbors_count(grid, vec)

    case on do
      true ->
        case num_neigbors do
          2 ->
            {vec, true}

          3 ->
            {vec, true}

          _ ->
            {vec, false}
        end

      false ->
        case num_neigbors do
          3 ->
            {vec, true}

          _ ->
            {vec, false}
        end
    end
  end

  def parse(input) do
    File.stream!(input)
    |> Enum.map(&String.trim/1)
    |> Enum.map(&String.graphemes/1)
    |> Enum.with_index()
    |> Enum.map(fn {li, i} ->
      Enum.with_index(li) |> Enum.map(fn {l, ii} -> {{ii, i}, l == "#"} end)
    end)
    |> Enum.to_list()
  end

  def output(grid) do
    for y <- grid do
      for({_, x} <- y, do: if(x, do: "#", else: ".")) |> Enum.join() |> IO.puts()
    end

    :ok
  end

  def get(grid, {x, y}) do
    r =
      Enum.find_value(grid, fn gg ->
        Enum.find(gg, fn
          {{^x, ^y}, _} = l -> l
          _ -> false
        end)
      end)

    case r do
      {_, val} -> val
      _ -> false
    end
  end

  def neighbors_count(grid, {x, y}) do
    for(i <- (x - 1)..(x + 1), ii <- (y - 1)..(y + 1), {i, ii} != {x, y}, do: {i, ii})
    |> Enum.map(&get(grid, &1))
    |> Enum.count(& &1)
  end
end
