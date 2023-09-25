defmodule Gift do
  def get_wrapping(file, paper \\ 0, ribbon_sum \\ 0) do
    data = IO.binread(file, :line)

    case data do
      :eof ->
        IO.puts("Paper Required: #{paper}")
        IO.puts("Ribbon Required: #{ribbon_sum}")

      _ ->
        lengths =
          data |> String.trim() |> String.split("x") |> Enum.map(fn x -> String.to_integer(x) end)

        lengths = Enum.sort(lengths)
        [l, w, h] = lengths
        sides = [l * w, l * h, w * h]
        surface_area = Enum.map(sides, &(2 * &1)) |> Enum.sum()
        slack = Enum.min(sides)

        ribbon = 2 * l + 2 * w + Enum.reduce(lengths, &(&1 * &2))

        get_wrapping(file, paper + surface_area + slack, ribbon_sum + ribbon)
    end
  end
end

case File.open("input.txt") do
  {:ok, file} -> Gift.get_wrapping(file)
  {:error, _} -> IO.puts("File Not Found")
end
