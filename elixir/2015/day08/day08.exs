defmodule AdventOfCode.Year2015.Day08 do
  @input "input.txt"

  def lengths() do
    File.stream!(@input)
    |> Enum.map(&String.trim/1)
    |> Enum.map(fn x -> {x, Code.eval_string(x), escape(x)} end)
    |> Enum.map(fn {org, {str, _}, encoded} ->
      %{
        str: str,
        length: String.length(org),
        eval_length: String.length(str),
        encode_length: String.length(encoded)
      }
    end)
  end

  defp escape(s) do
    e =
      String.replace(s, "\\", "\\\\")
      |> String.replace("\"", "\\\"")

    "\"" <> e <> "\""
  end
end

AdventOfCode.Year2015.Day08.lengths()
|> Enum.reduce(0, fn %{length: l, eval_length: e}, acc -> l - e + acc end)
|> IO.puts()

AdventOfCode.Year2015.Day08.lengths()
|> Enum.reduce(0, fn %{length: l, encode_length: e}, acc -> e - l + acc end)
|> IO.puts()
