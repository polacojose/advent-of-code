defmodule Day20Test do
  use ExUnit.Case
  doctest Day20

  test "Inifinite Delivery" do
    assert(Day20.infinite_delivery() |> Enum.take(1) |> List.last() == {1, 10})
    assert(Day20.infinite_delivery() |> Enum.take(2) |> List.last() == {2, 30})
    assert(Day20.infinite_delivery() |> Enum.take(3) |> List.last() == {3, 40})
    assert(Day20.infinite_delivery() |> Enum.take(4) |> List.last() == {4, 70})
    assert(Day20.infinite_delivery() |> Enum.take(5) |> List.last() == {5, 60})
    assert(Day20.infinite_delivery() |> Enum.take(6) |> List.last() == {6, 120})
    assert(Day20.infinite_delivery() |> Enum.take(7) |> List.last() == {7, 80})
    assert(Day20.infinite_delivery() |> Enum.take(8) |> List.last() == {8, 150})
    assert(Day20.infinite_delivery() |> Enum.take(9) |> List.last() == {9, 130})
  end

  test "Greater than or equal to" do
    assert(Day20.house_at_gifts_infinite(10) == {1, 10})
    assert(Day20.house_at_gifts_infinite(30) == {2, 30})
    assert(Day20.house_at_gifts_infinite(40) == {3, 40})
    assert(Day20.house_at_gifts_infinite(70) == {4, 70})
    assert(Day20.house_at_gifts_infinite(60) == {4, 70})
    assert(Day20.house_at_gifts_infinite(120) == {6, 120})
    assert(Day20.house_at_gifts_infinite(80) == {6, 120})
    assert(Day20.house_at_gifts_infinite(150) == {8, 150})
    assert(Day20.house_at_gifts_infinite(130) == {8, 150})
  end
end
