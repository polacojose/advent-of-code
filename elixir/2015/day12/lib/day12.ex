defmodule Day12 do
  def parse(option \\ "") do
    File.stream!("input.txt")
    |> Stream.map(&String.trim/1)
    |> Enum.map(&Jason.decode/1)
    |> Enum.map(fn {_, value} -> value end)
    |> Enum.map(fn x -> getNumbers(x, option) end)
    |> List.flatten()
    |> Enum.sum()
  end

  def getNumbers(o, option) do
    cond do
      is_list(o) ->
        Enum.map(o, &getNumbers(&1, option))

      is_map(o) ->
        values = Map.values(o)

        case option do
          :nonred ->
            case values |> Enum.find(nil, fn v -> v == "red" end) do
              nil ->
                Map.values(o) |> Enum.map(fn x -> getNumbers(x, option) end)

              _ ->
                0
            end

          _ ->
            Map.values(o) |> Enum.map(fn x -> getNumbers(x, option) end)
        end

      is_number(o) ->
        o

      true ->
        0
    end
  end
end

Day12.parse() |> IO.inspect()
Day12.parse(:nonred) |> IO.inspect()
