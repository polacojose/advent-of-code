defmodule Mode do
  def window(<<a::utf8, b::utf8, c::binary>>) do
    IO.puts("(#{<<a::utf8>>}, #{<<b::utf8>>})")
    window(<<b::utf8>> <> c)
  end

  def window(<<_::utf8, _::binary>>) do
  end

  def window(<<>>) do
  end
end

defmodule VowelCounter do
  @vowels ~r/(a|e|i|o|u)/i

  def count(""), do: 0

  def count(s) do
    @vowels |> Regex.scan(s) |> Enum.count()
  end

  def good_vowels(s), do: count(s) >= 3
end

defmodule DupCounter do
  def has_dup(s) do
    s
    |> String.graphemes()
    |> Enum.chunk_every(2, 1, :discard)
    |> Enum.map(fn x -> Enum.at(x, 0) == Enum.at(x, 1) end)
    |> Enum.map(fn x -> if x, do: 1, else: 0 end)
    |> Enum.sum() >= 1
  end

  def has_double_dup(s) do
    s
    |> String.codepoints()
    |> Enum.chunk_every(2, 1, :discard)
    |> Enum.map(fn x -> Regex.compile!("#{Enum.at(x, 0)}#{Enum.at(x, 1)}") end)
    |> Enum.find(nil, fn r ->
      Regex.scan(r, s) |> Enum.count() > 1
    end) != nil
  end

  def has_magic_three(s) do
    s
    |> String.codepoints()
    |> Enum.chunk_every(3, 1, :discard)
    |> Enum.find(nil, fn x -> Enum.at(x, 0) == Enum.at(x, 2) end) != nil
  end
end

defmodule BadString do
  @bad_str ~r/(ab|cd|pq|xy)/
  def has_bad_string(""), do: false

  def has_bad_string(s) do
    @bad_str |> Regex.scan(s) |> Enum.count() >= 1
  end
end

defmodule Niceness do
  def is_niceA(""), do: false

  def is_niceA(s) do
    VowelCounter.good_vowels(s) && DupCounter.has_dup(s) && !BadString.has_bad_string(s)
  end

  def is_niceB(""), do: false

  def is_niceB(s) do
    DupCounter.has_double_dup(s) && DupCounter.has_magic_three(s)
  end
end

File.stream!("input.txt")
|> Enum.map(&Niceness.is_niceA(&1))
|> Enum.map(fn x ->
  if x, do: 1, else: 0
end)
|> Enum.sum()
|> IO.puts()

File.stream!("input.txt")
|> Enum.map(&Niceness.is_niceB(&1))
|> Enum.map(fn x ->
  if x, do: 1, else: 0
end)
|> Enum.sum()
|> IO.puts()
