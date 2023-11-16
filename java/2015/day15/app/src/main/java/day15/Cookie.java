package day15;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;

public class Cookie {
    Map<Ingredient, Integer> ingredients;

    public Cookie(Map<Ingredient, Integer> ingredients) {
        this.ingredients = ingredients;
    }

    public int score() {
        return score(0);
    }

    public int score(int setCalories) {
        int capacity = 0;
        int durability = 0;
        int flavor = 0;
        int texture = 0;
        int calories = 0;
        for (Map.Entry<Ingredient, Integer> entry : ingredients.entrySet()) {
            Ingredient ingredient = entry.getKey();
            int amount = entry.getValue();
            capacity += ingredient.capacity * amount;
            durability += ingredient.durability * amount;
            flavor += ingredient.flavor * amount;
            texture += ingredient.texture * amount;
            calories += ingredient.calories * amount;
        }
        capacity = Math.max(0, capacity);
        durability = Math.max(0, durability);
        flavor = Math.max(0, flavor);
        texture = Math.max(0, texture);

        if (setCalories != 0 && calories != setCalories) {
            return 0;
        }

        return capacity * durability * flavor * texture;
    }

    public static int bestCookieScore(List<Ingredient> ingredientsList) {
        return bestCookieScore(ingredientsList, new java.util.HashMap<>(), 100, 0);
    }

    public static int bestCookieScore(List<Ingredient> ingredientsList, Map<Ingredient, Integer> cookieIngredients,
            int amount, int calories) {

        if (ingredientsList.size() == 1) {
            cookieIngredients.put(ingredientsList.get(0), amount);
            return new Cookie(cookieIngredients).score(calories);
        }

        int maxScore = 0;
        for (int i = 1; i < amount; i++) {
            List<Ingredient> ingredients = new ArrayList<>(ingredientsList);
            Ingredient ingredient = ingredients.remove(0);
            cookieIngredients.put(ingredient, i);
            maxScore = Math.max(maxScore, bestCookieScore(ingredients, cookieIngredients, amount - i, calories));
            cookieIngredients.remove(ingredient);
        }

        return maxScore;
    }

    public static int bestCookieScoreAtCalories(List<Ingredient> ingredientsList, int calories) {
        return bestCookieScore(ingredientsList, new java.util.HashMap<>(), 100, calories);
    }
}
