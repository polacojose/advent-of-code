defmodule Checker do
  def check() do
    check("")
  end

  def check(a, i \\ 0)

  def check(<<"000000", rest::binary>>, i) do
    IO.puts("Match 6 at #{i}")
  end

  def check(<<"00000", rest::binary>>, i) do
    IO.puts("Match 5 at #{i}")
    check("", i)
  end

  def check(_, i) do
    i = i + 1

    "iwrupvqb#{i}"
    |> :erlang.md5()
    |> Base.encode16()
    |> check(i)
  end
end

Checker.check()
