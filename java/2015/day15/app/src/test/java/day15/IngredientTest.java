package day15;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class IngredientTest {
    @Test
    void canParseIngredient() {
        Ingredient parsedIngredient = Ingredient
                .parseString("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8");
        Ingredient expectedIngredient = new Ingredient("Butterscotch", -1, -2, 6, 3, 8);
        assertTrue(parsedIngredient.equals(expectedIngredient));

        parsedIngredient = Ingredient
                .parseString("Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3");
        expectedIngredient = new Ingredient("Cinnamon", 2, 3, -2, -1, 3);
        assertTrue(parsedIngredient.equals(expectedIngredient));
    }
}
