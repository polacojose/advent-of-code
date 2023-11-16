package day15;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.ArrayList;
import java.util.List;
import java.util.Map;

import org.junit.jupiter.api.Test;

public class CookieTest {
    @Test
    void canScoreCookie() {
        Map<Ingredient, Integer> ingredients = new java.util.HashMap<>();
        ingredients.put(
                Ingredient.parseString("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8"),
                44);
        ingredients.put(Ingredient.parseString("Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"),
                56);
        Cookie cookie = new Cookie(ingredients);
        assertEquals(62842880, cookie.score());

    }

    @Test
    void canGetBestCookie() {
        List<Ingredient> ingredients = new ArrayList<>();
        ingredients.add(
                Ingredient.parseString("Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8"));
        ingredients
                .add(Ingredient.parseString("Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3"));
        int bestCookieScore = Cookie.bestCookieScore(ingredients);
        assertEquals(62842880, bestCookieScore);
    }
}
