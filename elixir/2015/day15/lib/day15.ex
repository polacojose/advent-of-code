defmodule Ingredient do
  defstruct name: "", capacity: 0, durability: 0, flavor: 0, texture: 0, calories: 0
end

defmodule Measurment do
  defstruct ingredient: %Ingredient{}, amount: 0
end

defmodule Day15 do
  @spec parse_ingredients(String.t()) :: list(Ingredient.t())
  def parse_ingredients(file) do
    File.stream!(file)
    |> Stream.map(&String.trim/1)
    |> Stream.map(&parse_ingredient/1)
    |> Enum.to_list()
  end

  @spec parse_ingredient(String.t()) :: Ingredient.t()
  def parse_ingredient(line) do
    name = Regex.run(~r/(\w+):/, line, capture: :all_but_first) |> hd

    Regex.scan(~r/(\w+) ([-\d]+)/, line, capture: :all_but_first)
    |> Enum.reduce(%Ingredient{name: name}, fn [k, v], acc ->
      Map.put(acc, String.to_atom(k), String.to_integer(v))
    end)
  end

  @spec score_cookie(list(Measurment.t())) :: integer
  def score_cookie(measurements) do
    Enum.map(
      measurements,
      fn %Measurment{
           ingredient: %Ingredient{
             capacity: c,
             durability: d,
             flavor: f,
             texture: t
           },
           amount: a
         } ->
        [c * a, d * a, f * a, t * a]
      end
    )
    |> Enum.zip()
    |> Enum.map(&Tuple.to_list/1)
    |> Enum.reduce(1, fn l, acc -> acc * max(Enum.sum(l), 0) end)
  end

  def best_cookie_score(file) do
    ingredients = parse_ingredients(file)
    combos = ingredient_combos(length(ingredients))

    Enum.map(combos, fn combo ->
      Tuple.to_list(combo)
      |> Enum.zip(ingredients)
      |> Enum.map(fn {amount, ingredient} ->
        %Measurment{ingredient: ingredient, amount: amount}
      end)
      |> score_cookie()
    end)
    |> Enum.max()
  end

  def ingredient_combos(n, target \\ 100, acc \\ [])

  def ingredient_combos(1, target, acc) do
    [target - Enum.sum(acc) | acc] |> List.to_tuple()
  end

  def ingredient_combos(n, target, acc) do
    Enum.map(1..target, fn x ->
      ingredient_combos(n - 1, target, [x | acc])
    end)
    |> List.flatten()
    |> Enum.filter(fn x ->
      !(x |> Tuple.to_list() |> Enum.any?(&(&1 <= 0)))
    end)
    |> Enum.filter(fn x ->
      x |> Tuple.to_list() |> Enum.sum() == target
    end)
  end
end
