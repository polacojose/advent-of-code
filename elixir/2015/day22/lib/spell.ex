defmodule Spell do
  defstruct type: :none, mana: 0, turns: 0

  @spec spells() :: [%__MODULE__{}]
  def spells() do
    [
      %__MODULE__{type: :magic_missle, mana: 53, turns: 0},
      %__MODULE__{type: :drain, mana: 73, turns: 0},
      %__MODULE__{type: :shield, mana: 113, turns: 6},
      %__MODULE__{type: :poison, mana: 173, turns: 6},
      %__MODULE__{type: :recharge, mana: 229, turns: 5}
    ]
  end

  @spec spell_lists(pos_integer()) :: [[%__MODULE__{}]]
  def spell_lists(length) do
    do_spell_lists(spells()) |> Enum.take(length) |> List.last()
  end

  defp do_spell_lists(spells) do
    spells = Enum.map(spells, &[&1])

    Stream.unfold(spells, fn
      acc ->
        {acc, for(s <- spells, a <- acc, do: s ++ a)}
    end)
  end
end
