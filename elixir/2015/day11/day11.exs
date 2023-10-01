defmodule AOC.Year2015.Day11 do
  @input "demo1-input.txt"

  def nextPassword(lpass) do
    nPass = AlphaIncrement.inc_alpha(lpass)

    case Eval.is_valid_password(nPass) do
      true -> nPass
      false -> nextPassword(nPass)
    end
  end
end

defmodule Eval do
  def is_valid_password(code_points) do
    has_all_good_chars(code_points) && has_triple(code_points) && has_two_pairs(code_points)
  end

  def has_all_good_chars(code_points) do
    Enum.find(code_points, nil, fn x ->
      x == ?i
    end) == nil &&
      Enum.find(code_points, nil, fn x -> x == ?o end) == nil &&
      Enum.find(code_points, nil, fn x -> x == ?l end) == nil
  end

  def has_two_pairs(_, pair_count \\ 0)
  def has_two_pairs(_, 2), do: true
  def has_two_pairs([], 1), do: false

  def has_two_pairs([a | t], pair_count) do
    case t do
      [b | t2] ->
        cond do
          a == b ->
            has_two_pairs(t2, pair_count + 1)

          true ->
            has_two_pairs([b] ++ t2, pair_count)
        end

      [] ->
        false
    end
  end

  def has_triple(_, num_match \\ 0)
  def has_triple(_, 2), do: true

  def has_triple([a | t], num_match) do
    case t do
      [b | t2] ->
        cond do
          a + 1 == b ->
            has_triple(t, num_match + 1)

          true ->
            has_triple([b] ++ t2, 0)
        end

      [] ->
        false
    end
  end
end

defmodule AlphaIncrement do
  def inc_alpha(s) do
    rev = to_charlist(s) |> Enum.reverse()
    inc(rev)
  end

  defp inc(a, carry \\ true, acc \\ [])

  defp inc([], false, acc) do
    Enum.reverse(acc)
  end

  defp inc([], true, acc) do
    inc([], false, acc ++ [?a])
  end

  defp inc([head | tail], carry, acc) do
    case carry do
      true ->
        head = head + 1

        case head > ?z do
          true ->
            head = ?a
            inc(tail, true, acc ++ [head])

          false ->
            acc = acc ++ [head]
            inc(tail, false, acc)
        end

      false ->
        inc(tail, carry, acc ++ [head])
    end
  end
end

start_password = "hxbxwxba"
passwordN1 = AOC.Year2015.Day11.nextPassword(start_password)
passwordN2 = AOC.Year2015.Day11.nextPassword(passwordN1)

IO.puts(passwordN1)
IO.puts(passwordN2)
