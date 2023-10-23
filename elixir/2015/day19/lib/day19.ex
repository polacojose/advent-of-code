defmodule Transformation do
  defstruct from: "", to: ""

  @spec parse(String.t()) :: %__MODULE__{}
  def parse(str) do
    [f, t] =
      String.split(str, " => ")

    %__MODULE__{from: f, to: t}
  end

  def parse_transformations(input) do
    File.stream!(input)
    |> Stream.map(&String.trim/1)
    |> Stream.map(&Transformation.parse/1)
    |> Enum.sort_by(&String.length(&1.to), :desc)
    |> Enum.to_list()
  end
end

defmodule Day19 do
  @moduledoc """
  Documentation for `Day19`.
  """

  @input_mol "CRnSiRnCaPTiMgYCaPTiRnFArSiThFArCaSiThSiThPBCaCaSiRnSiRnTiTiMgArPBCaPMgYPTiRnFArFArCaSiRnBPMgArPRnCaPTiRnFArCaSiThCaCaFArPBCaCaPTiTiRnFArCaSiRnSiAlYSiThRnFArArCaSiRnBFArCaCaSiRnSiThCaCaCaFYCaPTiBCaSiThCaSiThPMgArSiRnCaPBFYCaCaFArCaCaCaCaSiThCaSiRnPRnFArPBSiThPRnFArSiRnMgArCaFYFArCaSiRnSiAlArTiTiTiTiTiTiTiRnPMgArPTiTiTiBSiRnSiAlArTiTiRnPMgArCaFYBPBPTiRnSiRnMgArSiThCaFArCaSiThFArPRnFArCaSiRnTiBSiThSiRnSiAlYCaFArPRnFArSiThCaFArCaCaSiThCaCaCaSiRnPRnCaFArFYPMgArCaPBCaPBSiRnFYPBCaFArCaSiAl"

  @input_file "input.txt"

  def part1() do
    unique_transform_occurances(@input_mol, Transformation.parse_transformations(@input_file))
    |> Enum.count()
  end

  def part2() do
    transform_steps(@input_mol, Transformation.parse_transformations(@input_file))
  end

  @spec unique_transform_occurances(String.t(), [%Transformation{}]) :: [String.t()]
  def unique_transform_occurances(input, transforms) do
    for(
      t <- transforms,
      o <- do_replace_occurences(input, t),
      into: MapSet.new(),
      do: o
    )
  end

  def do_replace_occurences(input, transform) do
    case String.split(input, transform.from, parts: 2) do
      [h, t] ->
        cond do
          h == "" ->
            [transform.to <> t] ++
              for x <- do_replace_occurences(t, transform), do: transform.from <> x

          t == "" ->
            [h <> transform.to]

          true ->
            [h <> transform.to <> t] ++
              for x <- do_replace_occurences(t, transform), do: h <> transform.from <> x
        end

      _ ->
        []
    end
  end

  @spec transform_steps(String.t(), [%Transformation{}]) :: integer()
  def transform_steps(mole, transformations, steps \\ 0)
  def transform_steps("e", _, steps), do: steps

  def transform_steps(mole, transformations, steps) do
    trans = Enum.find(transformations, fn t -> mole =~ t.to end)

    transform_steps(
      String.replace(mole, trans.to, trans.from, global: false),
      transformations,
      steps + 1
    )
  end
end
