defmodule StringLength do
  @input "input.txt"

  def lengths() do
    File.stream!(@input)
    |> Enum.map(&String.trim/1)
    |> Enum.map(fn x -> {x, Code.eval_string(x), escaped(x)} end)
    |> Enum.map(fn {org, {str, _}, encoded} ->
      %{
        str: str,
        length: String.length(org),
        eval_length: String.length(str),
        encode_length: String.length(encoded)
      }
    end)
  end

  def escaped(s) do
    e =
      String.replace(s, "\\", "\\\\")
      |> String.replace("\"", "\\\"")

    "\"" <> e <> "\""
  end
end

StringLength.lengths()
|> Enum.reduce(0, fn %{length: l, eval_length: e}, acc -> l - e + acc end)
|> IO.puts()

StringLength.lengths()
|> Enum.reduce(0, fn %{length: l, encode_length: e}, acc -> e - l + acc end)
|> IO.puts()
