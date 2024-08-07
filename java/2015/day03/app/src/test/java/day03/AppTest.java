/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package day03;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class AppTest {
    @Test
    void canDetermineLocation() {
        Vector2 vec = new Vector2(0, 0);
        assertTrue(vec.up().equals(new Vector2(0, -1)));
        assertTrue(vec.right().equals(new Vector2(1, -1)));
        assertTrue(vec.down().equals(new Vector2(1, 0)));
        assertTrue(vec.left().equals(new Vector2(0, 0)));
    }

    @Test
    void canCountVisitedHouses() {
        assertTrue(Delivery.visitedHomes("") == 1);
        assertTrue(Delivery.visitedHomes(">") == 2);
        assertTrue(Delivery.visitedHomes("^>v<") == 4);
        assertTrue(Delivery.visitedHomes("^v^v^v^v^v") == 2);
    }

    @Test
    void canCountVisitedHousesWithRoboSanta() {
        assertTrue(Delivery.visitedHomesWithRoboSanta("^v") == 3);
        assertTrue(Delivery.visitedHomesWithRoboSanta("^>v<") == 3);
        assertTrue(Delivery.visitedHomesWithRoboSanta("^v^v^v^v^v") == 11);
    }
}
