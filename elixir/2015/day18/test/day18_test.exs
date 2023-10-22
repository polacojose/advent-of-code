defmodule Day18Test do
  use ExUnit.Case
  doctest Day18

  test "Can display" do
    grid = Day18.parse_grid_file("demo-input.txt")
    assert(Day18.output(grid), ".#.#.#\n...##.\n#....#\n..#...\n#.#..#\n####..\n")
  end

  test "Can count grid" do
    grid = Day18.parse_grid_file("demo-input.txt")
    out = Day18.how_many_lights_on?(grid)
    assert(Day18.how_many_lights_on?(grid) == 15)
  end

  test "Can step" do
    grid = Day18.parse_grid_file("demo-input.txt")
    grid = Enum.reduce(1..4, grid, fn _, g -> Day18.step_grid(g) end)
    assert(Day18.how_many_lights_on?(grid) == 4)
  end

  test "Can step stuck" do
    grid = Day18.parse_grid_file("demo-input.txt") |> Day18.stick_corners_on()
    IO.puts(grid |> Day18.output())
    grid = Enum.reduce(1..5, grid, fn _, g -> Day18.step_grid(g) |> Day18.stick_corners_on() end)
    assert(Day18.how_many_lights_on?(grid) == 17)
  end
end
