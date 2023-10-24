defmodule Item do
  @type mode :: :weapon | :armor | :ring
  defstruct name: "", cost: 0, damage: 0, armor: 0, type: :none

  @spec parse_items(String.t()) :: [%__MODULE__{}]
  def parse_items(list) do
    (File.stream!(list) |> Enum.map(&String.trim/1) |> do_parse_items()) ++ optional_items()
  end

  def optional_items() do
    [
      %Item{name: "No Armor", cost: 0, damage: 0, armor: 0, type: :armor},
      %Item{name: "No Ring1", cost: 0, damage: 0, armor: 0, type: :ring},
      %Item{name: "No Ring2", cost: 0, damage: 0, armor: 0, type: :ring}
    ]
  end

  @spec do_parse_items([String.t()]) :: [%__MODULE__{}]
  defp do_parse_items(lines, mode \\ :weapon)
  defp do_parse_items([], _), do: []
  defp do_parse_items(["" | t], _), do: do_parse_items(t, :weapon)
  defp do_parse_items(["Weapons:" <> _ | t], _), do: do_parse_items(t, :weapon)
  defp do_parse_items(["Armor:" <> _ | t], _), do: do_parse_items(t, :armor)
  defp do_parse_items(["Rings:" <> _ | t], _), do: do_parse_items(t, :ring)

  defp do_parse_items([h | t], mode) do
    [n, c, d, a] =
      Regex.run(~r/(\w+[\s*\+\d]*?)\s+(\d+)\s+(\d)\s+(\d)/, h, capture: :all_but_first)

    [
      %Item{
        name: n,
        cost: String.to_integer(c),
        damage: String.to_integer(d),
        armor: String.to_integer(a),
        type: mode
      }
    ] ++
      do_parse_items(t, mode)
  end
end
