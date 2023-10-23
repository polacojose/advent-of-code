defmodule Day19Test do
  use ExUnit.Case
  doctest Day19

  @demo_input_mol "HOH"

  test "Get occurences using demo molecule" do
    occurences =
      Day19.unique_transform_occurances(
        "HOH",
        Transformation.parse_transformations("demo-input.txt")
      )

    assert(length(occurences) == 4)
  end

  test "Get steps using demo molecule" do
    steps =
      Day19.transform_steps(
        "HOH",
        Transformation.parse_transformations("demo2-input.txt")
      )

    assert(steps == 3)
  end
end
