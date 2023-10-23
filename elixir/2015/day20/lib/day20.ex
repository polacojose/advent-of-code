defmodule Day20 do
  @gift_limit 33_100_000

  @spec house_at_gifts_infinite(integer()) :: {integer(), integer()}
  def house_at_gifts_infinite(gifts) do
    Enum.find(infinite_delivery(), fn {_, p} -> p >= gifts end)
  end

  @spec house_at_gifts_finite(integer()) :: {integer(), integer()}
  def house_at_gifts_finite(gifts) do
    Enum.find(infinite_delivery([:limit]), fn {_, p} -> p >= gifts end)
  end

  def infinite_delivery(opts \\ []) do
    delivery_amount = if :limit in opts, do: 11, else: 10

    Stream.unfold({1, 10}, fn {h, acc} ->
      {{h, acc},
       {h + 1, factors(h + 1, opts) |> Enum.map(fn x -> x * delivery_amount end) |> Enum.sum()}}
    end)
  end

  @spec factors(integer(), [keyword()]) :: [integer()]
  def factors(t, opts \\ []) do
    [t | do_factors(t, :math.sqrt(t), opts)] |> Enum.uniq()
  end

  @spec do_factors(integer(), float(), [keyword()]) :: [integer()]
  defp do_factors(t, max, opts, p \\ 1)
  defp do_factors(_, max, _, p) when p > max, do: []

  defp do_factors(t, max, opts, p) do
    case rem(t, p) == 0 do
      true ->
        r = div(t, p)

        if :limit in opts do
          pre = if p * 50 >= t, do: [p], else: []
          pre = pre ++ if r * 50 >= t, do: [r], else: []
          pre ++ do_factors(t, max, opts, p + 1)
        else
          [p] ++ [r] ++ do_factors(t, max, opts, p + 1)
        end

      false ->
        do_factors(t, max, opts, p + 1)
    end
  end
end
