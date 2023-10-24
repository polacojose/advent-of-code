defmodule PlayerStats do
  @moduledoc """
  Defines and parses player stats and loadouts
  """
  defstruct health: 0, damage: 0, armor: 0

  @type loadout :: [%Item{}]

  @spec from_loadout(loadout(), non_neg_integer) :: %__MODULE__{}
  def from_loadout(loadout, health) do
    Enum.reduce(loadout, %__MODULE__{health: health}, fn i, stats ->
      %{stats | damage: stats.damage + i.damage, armor: stats.armor + i.armor}
    end)
  end

  def player_loadouts() do
    items = Item.parse_items("input.txt")

    for %{type: :weapon} = w <- items,
        %{type: :armor} = a <- items,
        %{type: :ring} = lr <- items,
        %{type: :ring} = rr <- items,
        lr != rr,
        do: [w, a, lr, rr]
  end

  @spec sort_loadouts_with_cost([loadout()]) :: [loadout()]
  def sort_loadouts_with_cost(loadouts, sorter \\ :asc) do
    Enum.map(loadouts, fn lo -> {loadout_cost(lo), lo} end)
    |> Enum.sort_by(&(&1 |> elem(0)), sorter)
  end

  @spec loadout_cost(loadout()) :: non_neg_integer()
  def loadout_cost(loadout) do
    Enum.reduce(loadout, 0, fn l, acc -> l.cost + acc end)
  end
end
