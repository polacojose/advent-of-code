defmodule Day14Test do
  use ExUnit.Case
  doctest Day14

  @racers [
    %Reindeer{name: "Comet", speed: 14, fly_time: 10, rest_time: 127},
    %Reindeer{name: "Dancer", speed: 16, fly_time: 11, rest_time: 162}
  ]

  test "demo race" do
    assert Day14.race(@racers, 1000) ==
             {1120, %Reindeer{name: "Comet", speed: 14, fly_time: 10, rest_time: 127}}
  end
end
