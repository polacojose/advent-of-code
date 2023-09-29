import Bitwise

defmodule AdventOfCode.Year2015.Day07.Part1 do
  @input "input.txt"

  def parse() do
    File.stream!(@input)
    |> Enum.map(&String.trim/1)
    |> Enum.map(&String.split/1)
    |> Enum.map(&parse_inst/1)
  end

  def parse_inst([a, "->", r]), do: to_inst(:assign, [a], r)
  def parse_inst(["NOT", a, "->", r]), do: to_inst(:not, [a], r)
  def parse_inst([a, "AND", b, "->", r]), do: to_inst(:and, [a, b], r)
  def parse_inst([a, "OR", b, "->", r]), do: to_inst(:or, [a, b], r)
  def parse_inst([a, "LSHIFT", b, "->", r]), do: to_inst(:lshift, [a, b], r)
  def parse_inst([a, "RSHIFT", b, "->", r]), do: to_inst(:rshift, [a, b], r)

  def get_val(a, map) do
    val = Integer.parse(a)

    case val do
      :error -> Map.get(map, a, 0)
      {v, _} -> v
    end
  end

  def to_inst(type, opers, result) do
    {type, opers, result}
  end

  def eval_oper(oper, map, dest, instructs) do
    result = Map.get(map, oper)

    case result do
      nil ->
        val = Integer.parse(oper)

        case val do
          :error ->
            instruct = Enum.find(instructs, nil, fn {_, _, result} -> result == oper end)
            eval(instruct, map, instructs)

          {v, _} ->
            Map.put(map, dest, v)
        end

      _ ->
        map
    end
  end

  def eval({type, opers, result}, map, instructs) do
    map =
      opers
      |> Enum.reduce(map, fn o, m ->
        eval_oper(o, m, result, instructs)
      end)

    opers =
      opers
      |> Enum.map(fn x ->
        get_val(x, map)
      end)

    case type do
      :assign ->
        Map.put(map, result, Enum.at(opers, 0))

      :not ->
        Map.put(map, result, 65535 - Enum.at(opers, 0))

      :and ->
        Map.put(map, result, band(Enum.at(opers, 0), Enum.at(opers, 1)))

      :or ->
        Map.put(map, result, bor(Enum.at(opers, 0), Enum.at(opers, 1)))

      :lshift ->
        Map.put(map, result, bsl(Enum.at(opers, 0), Enum.at(opers, 1)))

      :rshift ->
        Map.put(map, result, bsr(Enum.at(opers, 0), Enum.at(opers, 1)))
    end
  end
end

instructs = AdventOfCode.Year2015.Day07.Part1.parse()

%{"a" => a} =
  AdventOfCode.Year2015.Day07.Part1.eval(
    Enum.find(instructs, nil, fn {_, _, result} -> "a" == result end),
    %{},
    instructs
  )

a |> IO.inspect()

%{"a" => a} =
  AdventOfCode.Year2015.Day07.Part1.eval(
    Enum.find(instructs, nil, fn {_, _, result} -> "a" == result end),
    %{"b" => a},
    instructs
  )

a |> IO.inspect()
