defmodule Nav do
  def do_navigate(a, vec \\ {0, 0}, locs \\ MapSet.new([{0, 0}]))
  def do_navigate_switch(a, vecA \\ {0, 0}, vecB \\ {0, 0}, i \\ 0, locs \\ MapSet.new([{0, 0}]))

  def do_navigate(<<>>, _, locs) do
    Enum.count(locs) |> IO.puts()
  end

  def do_navigate(<<c::utf8, rest::binary>>, vec, locs) do
    cur = vec_diff(vec, c)
    locs = MapSet.put(locs, cur)
    do_navigate(rest, cur, locs)
  end

  def do_navigate_switch(<<>>, _, _, _, locs) do
    Enum.count(locs) |> IO.puts()
  end

  def do_navigate_switch(<<c::utf8, rest::binary>>, vecA, vecB, i, locs) when rem(i, 2) != 0 do
    vecB = vec_diff(vecB, c)
    locs = MapSet.put(locs, vecB)
    do_navigate_switch(rest, vecA, vecB, i + 1, locs)
  end

  def do_navigate_switch(<<c::utf8, rest::binary>>, vecA, vecB, i, locs) when rem(i, 2) == 0 do
    vecA = vec_diff(vecA, c)
    locs = MapSet.put(locs, vecA)
    do_navigate_switch(rest, vecA, vecB, i + 1, locs)
  end

  def vec_diff({x, y}, dir) do
    case dir do
      ?^ -> {x, y - 1}
      ?< -> {x - 1, y}
      ?v -> {x, y + 1}
      ?> -> {x + 1, y}
      _ -> {x, y}
    end
  end
end

File.read!("input.txt") |> Nav.do_navigate()
File.read!("input.txt") |> Nav.do_navigate_switch()
