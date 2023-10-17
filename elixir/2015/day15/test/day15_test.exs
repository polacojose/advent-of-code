defmodule Day15Test do
  use ExUnit.Case
  doctest Day15

  test "Parses ingredient" do
    assert Day15.parse_ingredient(
             "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8"
           ) == %Ingredient{
             name: "Butterscotch",
             capacity: -1,
             durability: -2,
             flavor: 6,
             texture: 3,
             calories: 8
           }
  end

  test "Scores Cookie" do
    assert Day15.score_cookie([
             %Measurment{
               ingredient: %Ingredient{
                 name: "Butterscotch",
                 capacity: -1,
                 durability: -2,
                 flavor: 6,
                 texture: 3,
                 calories: 8
               },
               amount: 44
             },
             %Measurment{
               ingredient: %Ingredient{
                 name: "Cinnamon",
                 capacity: 2,
                 durability: 3,
                 flavor: -2,
                 texture: -1,
                 calories: 3
               },
               amount: 56
             }
           ]) == 62_842_880
  end

  test "Sum calories" do
    assert Day15.cookie_calories([
             %Measurment{
               ingredient: %Ingredient{
                 name: "Butterscotch",
                 calories: 8
               },
               amount: 44
             },
             %Measurment{
               ingredient: %Ingredient{
                 name: "Cinnamon",
                 calories: 3
               },
               amount: 56
             }
           ]) == 520
  end
end
