defmodule Day16Test do
  use ExUnit.Case
  doctest Day16

  test "parses aunt" do
    assert Aunt.parse("Sue 69: vizslas: 5, perfumes: 3, cars: 9") == %{
             id: 69,
             vizslas: 5,
             perfumes: 3,
             cars: 9
           }
  end
end
