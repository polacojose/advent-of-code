defmodule Reindeer do
  defstruct name: "", points: 0, speed: 0, fly_time: 0, rest_time: 0

  @moduledoc """
  Documentation for `Reindeer`.
  """
  @spec dist_at_t(%Reindeer{}, integer) :: integer
  def dist_at_t(r, seconds) do
    period_length = r.fly_time + r.rest_time
    periods = div(seconds, period_length)
    dist = r.speed * r.fly_time * periods
    seconds = seconds - periods * period_length
    dist + min(seconds, r.fly_time) * r.speed
  end

  @spec parse_stream(File.Stream.t()) :: [%Reindeer{}]
  def parse_stream(stream) do
    Stream.map(stream, &parse_reindeer/1)
    |> Enum.to_list()
  end

  @spec parse_reindeer(String.t()) :: %Reindeer{}
  defp parse_reindeer(string) do
    string = String.trim(string) |> String.split()
    [name, _, _, speed, _, _, fly_time, _, _, _, _, _, _, rest_time, _] = string

    %Reindeer{
      name: name,
      points: 0,
      speed: String.to_integer(speed),
      fly_time: String.to_integer(fly_time),
      rest_time: String.to_integer(rest_time)
    }
  end
end

defmodule Day14 do
  def do_races() do
    racers =
      File.stream!("input.txt")
      |> Reindeer.parse_stream()

    %{part1: race_final(racers, 2503), part2: race_progessive_points(racers, 2503)}
  end

  @moduledoc """
  Documentation for `Day14`.
  """
  @spec race_final([%Reindeer{}], integer) :: {integer, %Reindeer{}}
  def race_final(racers, length) do
    Enum.reduce(racers, {0, nil}, fn r, {d, last} ->
      dist = Reindeer.dist_at_t(r, length)
      if dist > d, do: {dist, r}, else: {d, last}
    end)
  end

  @spec race_progessive_points([%Reindeer{}], integer) :: {integer, %Reindeer{}}
  def race_progessive_points(racers, length) do
    Enum.reduce(1..length, racers, fn n, acc ->
      lead_dist =
        Enum.reduce(racers, 0, fn r, d ->
          dist = Reindeer.dist_at_t(r, n)
          if dist > d, do: dist, else: d
        end)

      Enum.map(acc, fn r ->
        dist = Reindeer.dist_at_t(r, n)
        if dist == lead_dist, do: %{r | points: r.points + 1}, else: r
      end)
    end)
    |> Enum.max_by(& &1.points)
  end
end
