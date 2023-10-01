defmodule AOC.Year2015.Day10 do
  def look_and_say_iter(string, 0), do: string

  def look_and_say_iter(string, iter) do
    look_and_say_iter(look_and_say(string), iter - 1)
  end

  defp look_and_say(string) do
    String.codepoints(string)
    |> count_chars()
  end

  defp count_chars([h | t]) do
    count_chars(t, h, 1, "")
  end

  defp count_chars([], l, _, _) do
    "1" <> l
  end

  defp count_chars([h], l, num, acc) do
    case h do
      ^l ->
        acc <> Integer.to_string(num + 1) <> l

      _ ->
        acc = acc <> Integer.to_string(num) <> l
        acc <> Integer.to_string(1) <> h
    end
  end

  defp count_chars([h | t], l, num, acc) do
    case h do
      ^l ->
        count_chars(t, h, num + 1, acc)

      _ ->
        acc = acc <> Integer.to_string(num) <> l
        count_chars(t, h, 1, acc)
    end
  end
end

File.stream!("input.txt")
|> Enum.map(&String.trim/1)
|> Enum.map(fn x ->
  AOC.Year2015.Day10.look_and_say_iter(x, 40)
end)
|> Enum.map(&String.length/1)
|> IO.inspect()

File.stream!("input.txt")
|> Enum.map(&String.trim/1)
|> Enum.map(fn x ->
  AOC.Year2015.Day10.look_and_say_iter(x, 50)
end)
|> Enum.map(&String.length/1)
|> IO.inspect()
