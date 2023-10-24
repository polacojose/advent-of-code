defmodule BattleSim do
  @moduledoc """
  Simulates the battle
  """
  defstruct effects: [], player: nil, boss: nil
  @type state :: %__MODULE__{}
  @type phase :: :player | :boss

  @spec new_state(%Stats{}, %Stats{}) :: %__MODULE__{}
  def new_state(player, boss) do
    %__MODULE__{player: player, boss: boss}
  end

  @spec battle(%__MODULE__{}, mode :: :normal | :hard) ::
          :win | :loss_spells | :loss_mana | :loss_health
  def battle(state \\ %__MODULE__{}, mode \\ :normal, phase \\ :player)

  # Player loses immediately if mana is negative
  def battle(%__MODULE__{player: %Stats{mana: m}}, _, _) when m < 0, do: :loss_mana

  def battle(%__MODULE__{player: %Stats{health: h}}, :hard, _) when h <= 0, do: :loss_health
  def battle(%__MODULE__{boss: %Stats{health: h}}, _, _) when h <= 0, do: :win
  def battle(%__MODULE__{player: %Stats{health: h}}, _, :player) when h <= 0, do: :loss_health
  def battle(%__MODULE__{player: %Stats{mana: m}}, _, :player) when m == 0, do: :loss_mana

  def battle(%__MODULE__{player: %Stats{spells: []}}, _, :player), do: :loss_spells

  @print false

  # Player Phase
  def battle(state, mode, :player) do
    if @print do
      %__MODULE__{player: player, boss: boss, effects: effects} = state

      effect_out = Enum.map(effects, fn e -> "#{e.type}:#{e.turns}" end) |> Enum.join(", ")

      IO.puts("""

      -- Player turn --
      - Player has #{player.health} hit points, #{player.armor} armor, #{player.mana} mana
      - Boss has #{boss.health} hit points
      - Effects: #{effect_out}
      """)
    end

    state =
      if mode == :hard do
        %{state | player: %{state.player | health: state.player.health - 1}}
      else
        state
      end

    if mode != :hard or state.player.health > 0 do
      state = resolve_effects(state)
      state = cast_spell(state)
      battle(state, mode, :boss)
    else
      state
    end
  end

  # Boss Phase
  def battle(state, mode, :boss) do
    if @print do
      %__MODULE__{player: player, boss: boss, effects: effects} = state
      effect_out = Enum.map(effects, fn e -> "#{e.type}:#{e.turns}" end) |> Enum.join(", ")

      IO.puts("""

      -- Boss turn --
      - Player has #{player.health} hit points, #{player.armor} armor, #{player.mana} mana
      - Boss has #{boss.health} hit points
      - Effects: #{effect_out}
      """)
    end

    state = resolve_effects(state)
    %__MODULE__{player: player, boss: boss} = state
    attack = max(boss.damage - player.armor, 1)
    player = %{player | health: player.health - attack}

    if @print do
      IO.puts("Boss deals #{attack} damage.")
    end

    battle(%{state | player: player}, mode, :player)
  end

  @spec cast_spell(%__MODULE__{}) :: %__MODULE__{}
  def cast_spell(state) do
    %__MODULE__{player: player, boss: boss, effects: effects} = state
    spell = player.spells |> hd
    player = %{player | spells: player.spells |> tl}

    case spell.type do
      :magic_missle ->
        player = %{player | mana: player.mana - 53}
        boss = %{boss | health: boss.health - 4}

        if @print do
          IO.puts("Player casts Magic Missile dealing 4 damage.")
        end

        state = %{state | player: player}
        state = %{state | boss: boss}
        %{state | effects: effects}

      :drain ->
        player = %{player | mana: player.mana - 73}
        boss = %{boss | health: boss.health - 2}
        player = %{player | health: player.health + 2}

        if @print do
          IO.puts("Player casts Drain, dealing 2 damage, and healing 2 hit points.")
        end

        state = %{state | player: player}
        state = %{state | boss: boss}
        %{state | effects: effects}

      :shield ->
        player = %{player | mana: player.mana - 113}
        state = %{state | player: player}

        if !Enum.any?(effects, fn %{type: t} -> t == spell.type end) do
          player = %{player | armor: player.armor + 7}
          effects = [spell | effects]

          if @print do
            IO.puts("Player casts Shield, increasing armor by 7.")
          end

          state = %{state | player: player}
          state = %{state | boss: boss}
          %{state | effects: effects}
        else
          state
        end

      :poison ->
        player = %{player | mana: player.mana - 173}
        state = %{state | player: player}

        if !Enum.any?(effects, fn %{type: t} -> t == spell.type end) do
          effects = [spell | effects]

          if @print do
            IO.puts("Player casts Poison.")
          end

          state = %{state | player: player}
          state = %{state | boss: boss}
          %{state | effects: effects}
        else
          state
        end

      :recharge ->
        player = %{player | mana: player.mana - 229}
        state = %{state | player: player}

        if !Enum.any?(effects, fn %{type: t} -> t == spell.type end) do
          effects = [spell | effects]

          if @print do
            IO.puts("Player casts Recharge.")
          end

          state = %{state | player: player}
          state = %{state | boss: boss}
          %{state | effects: effects}
        else
          state
        end
    end
  end

  @spec resolve_effects(%__MODULE__{}) :: %__MODULE__{}
  def resolve_effects(state) do
    %__MODULE__{effects: effects} = state
    state = %{state | effects: []}

    Enum.reduce(effects, state, fn spell, state ->
      case spell.type do
        :shield ->
          spell = %{spell | turns: spell.turns - 1}

          if @print do
            IO.puts("Shields timer is now #{spell.turns}." |> String.capitalize())
          end

          if spell.turns > 0 do
            effects = [spell | state.effects]
            %{state | effects: effects}
          else
            player = %{state.player | armor: state.player.armor - 7}
            %{state | player: player}
          end

        :poison ->
          spell = %{spell | turns: spell.turns - 1}
          boss = %{state.boss | health: state.boss.health - 3}
          state = %{state | boss: boss}

          if @print do
            IO.puts(
              "#{spell.type} deals 3 damage; it's timer is now #{spell.turns}."
              |> String.capitalize()
            )
          end

          if spell.turns > 0 do
            effects = [spell | state.effects]
            %{state | effects: effects}
          else
            state
          end

        :recharge ->
          spell = %{spell | turns: spell.turns - 1}
          player = %{state.player | mana: state.player.mana + 101}
          state = %{state | player: player}

          if spell.turns > 0 do
            effects = [spell | state.effects]
            %{state | effects: effects}
          else
            state
          end
      end
    end)
  end
end
