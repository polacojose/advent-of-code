defmodule Aunt do
  @spec parse(String.t()) :: %{}
  def parse(line) do
    [id | stats] = Regex.run(~r/Sue\s+(\d+):/, line, capture: :all_but_first)

    Regex.scan(~r/(\w+):\s+(\d+)/, line, capture: :all_but_first)
    |> Enum.reduce(%{id: String.to_integer(id)}, fn [k, v], aunt ->
      Map.put(aunt, String.to_atom(k), String.to_integer(v))
    end)
  end
end

defmodule Day16 do
  @match %{
    children: 3,
    cats: 7,
    samoyeds: 2,
    pomeranians: 3,
    akitas: 0,
    vizslas: 0,
    goldfish: 5,
    trees: 3,
    cars: 2,
    perfumes: 1
  }

  @input_file "input.txt"

  def part1_which_aunt?() do
    File.stream!(@input_file)
    |> Stream.map(&Aunt.parse/1)
    |> Stream.filter(&part1_comparison(&1, @match))
    |> Enum.to_list()
  end

  def part2_which_aunt?() do
    File.stream!(@input_file)
    |> Stream.map(&Aunt.parse/1)
    |> Stream.filter(&part2_comparison(&1, @match))
    |> Enum.to_list()
  end

  @spec part1_comparison(Map.t(), Map.t()) :: boolean
  defp part1_comparison(aunt, target) do
    Enum.all?(
      Map.keys(aunt),
      fn k ->
        if k == :id do
          true
        else
          Map.get(aunt, k) == Map.get(target, k)
        end
      end
    )
  end

  @spec part2_comparison(Map.t(), Map.t()) :: boolean
  defp part2_comparison(aunt, target) do
    Enum.all?(
      Map.keys(aunt),
      fn k ->
        cond do
          k == :id ->
            true

          k in [:cats, :trees] ->
            Map.get(aunt, k) > Map.get(target, k)

          k in [:pomeranians, :goldfish] ->
            Map.get(aunt, k) < Map.get(target, k)

          true ->
            Map.get(aunt, k) == Map.get(target, k)
        end
      end
    )
  end
end
