defmodule Count do
  def sum_chars(chars, sum \\ 0) do
    <<char, rest::binary>> = chars

    case char do
      ?( ->
        sum_chars(rest, sum + 1)

      ?) ->
        sum_chars(rest, sum + -1)

      _ ->
        sum
    end
  end

  def is_basement(chars, i \\ 0, sum \\ 0) do
    <<char, rest::binary>> = chars

    case sum do
      -1 ->
        i

      _ ->
        case char do
          ?( ->
            is_basement(rest, i + 1, sum + 1)

          ?) ->
            is_basement(rest, i + 1, sum + -1)

          _ ->
            sum
        end
    end
  end
end

file = File.read!("input.txt")
IO.puts("Ended at #{Count.sum_chars(file)}")
IO.puts("Entered basement at #{Count.is_basement(file)}")
