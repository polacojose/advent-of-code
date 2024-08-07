/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package day15;

import java.util.Arrays;
import java.util.List;

import filelinesloader.FileLinesLoader;

public class App {
    public static void main(String[] args) {
        Ingredient[] ingredients = getIngredients();
        part1(ingredients);
        part2(ingredients);
    }

    static Ingredient[] getIngredients() {
        String[] lines = FileLinesLoader.getLines("input.txt");
        Ingredient[] ingredients = new Ingredient[lines.length];
        for (int i = 0; i < lines.length; i++) {
            ingredients[i] = Ingredient.parseString(lines[i]);
        }
        return ingredients;
    }

    private static void part1(Ingredient[] ingredients) {
        List<Ingredient> ingredientsList = Arrays.asList(ingredients);
        int bestCookieScore = Cookie.bestCookieScore(ingredientsList);
        System.out.println(String.format("Best Cookie Score: %d", bestCookieScore));
    }

    private static void part2(Ingredient[] ingredients) {
        List<Ingredient> ingredientsList = Arrays.asList(ingredients);
        int bestCookieScore = Cookie.bestCookieScoreAtCalories(ingredientsList, 500);
        System.out.println(String.format("Best Cookie Score: %d", bestCookieScore));
    }
}
