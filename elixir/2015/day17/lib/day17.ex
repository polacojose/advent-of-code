defmodule Day17 do
  def containers() do
    "input.txt"
    |> File.read!()
    |> String.split("\n", trim: true)
    |> Enum.map(&String.to_integer/1)
  end

  @spec part1(non_neg_integer()) :: non_neg_integer()
  def part1(target) do
    containers = containers()

    Enum.reduce(combinations(containers), 0, fn x, acc ->
      if Enum.sum(x) == target, do: acc + 1, else: acc
    end)
  end

  @spec part2(non_neg_integer()) :: non_neg_integer()
  def part2(target) do
    results =
      Enum.reduce(combinations(containers()), [], fn x, acc ->
        sum = Enum.sum(x)
        if sum == target, do: [{length(x), sum} | acc], else: acc
      end)

    minimum = Enum.min_by(results, fn {length, _} -> length end) |> elem(0)
    Enum.filter(results, fn {length, _} -> length == minimum end) |> Enum.count()
  end

  @spec combinations(list) :: list
  defp combinations([]), do: []

  defp combinations([head | tail]) do
    tail_combinations = combinations(tail)
    merged_combinations = Enum.map([[]] ++ tail_combinations, &[head | &1])
    tail_combinations ++ merged_combinations
  end
end
