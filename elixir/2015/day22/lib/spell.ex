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
end
