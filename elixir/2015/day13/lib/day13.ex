defmodule Day13 do
  def get_names(input) do
    File.stream!(input)
    |> Stream.map(&String.trim/1)
    |> Stream.map(&String.split/1)
    |> Stream.map(fn x ->
      Enum.at(x, 0)
    end)
    |> Enum.to_list()
    |> Enum.uniq()
  end

  def get_happiness_map(input) do
    File.stream!(input)
    |> Stream.map(&String.trim/1)
    |> Stream.map(&String.replace(&1, ".", ""))
    |> Stream.map(&String.split/1)
    |> Stream.map(fn x ->
      happiness = String.to_integer(Enum.at(x, 3))

      case Enum.at(x, 2) do
        "gain" -> {Enum.at(x, 0), Enum.at(x, 10), happiness}
        "lose" -> {Enum.at(x, 0), Enum.at(x, 10), happiness * -1}
      end
    end)
    |> Enum.reduce(%{}, fn {s, d, h}, acc ->
      Map.put(acc, {s, d}, h)
    end)
  end
end

defmodule Permutations do
  def permutations([]), do: [[]]

  def permutations(list) do
    for elem <- list, rest <- permutations(list -- [elem]), do: [elem | rest]
  end
end

defmodule GetHappiness do
  def sum_happiness([h | t], map) do
    names = [h | t] ++ [h]

    Enum.chunk_every(names, 2, 1, :discard)
    |> Enum.map(fn [a, b] ->
      Map.get(map, {a, b}) + Map.get(map, {b, a})
    end)
    |> Enum.sum()
  end
end

input = "input1.txt"
map = Day13.get_happiness_map(input)

Day13.get_names(input)
|> Permutations.permutations()
|> Enum.map(fn n ->
  GetHappiness.sum_happiness(
    n,
    map
  )
end)
|> Enum.max()
|> IO.inspect()

input = "input2.txt"
map = Day13.get_happiness_map(input)

Day13.get_names(input)
|> Permutations.permutations()
|> Enum.map(fn n ->
  GetHappiness.sum_happiness(
    n,
    map
  )
end)
|> Enum.max()
|> IO.inspect()
