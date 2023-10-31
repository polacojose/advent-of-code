package day03;

import java.util.HashSet;
import java.util.Set;

public class Delivery {
    public static int visitedHomes(String directions) throws IllegalArgumentException {
        directions = directions.trim();
        Vector2 vector = new Vector2(0, 0);
        Set<Vector2> visited = new HashSet<>();
        visited.add(vector.copy());
        for (char dir : directions.toCharArray()) {
            move(vector, dir);
            visited.add(vector.copy());
        }
        return visited.size();
    }

    public static int visitedHomesWithRoboSanta(String directions) throws IllegalArgumentException {
        directions = directions.trim();

        Vector2 santa = new Vector2(0, 0);
        Vector2 roboSanta = new Vector2(0, 0);

        Set<Vector2> visited = new HashSet<>();
        visited.add(santa.copy());

        int i = 0;
        for (char dir : directions.toCharArray()) {
            if (i++ % 2 == 0) {
                move(santa, dir);
                visited.add(santa.copy());
            } else {
                move(roboSanta, dir);
                visited.add(roboSanta.copy());
            }
        }
        return visited.size();
    }

    private static void move(Vector2 vec, char dir) {
        switch (dir) {
            case '^':
                vec.up();
                break;
            case 'v':
                vec.down();
                break;
            case '<':
                vec.left();
                break;
            case '>':
                vec.right();
                break;
            default:
                System.out.println(dir);
                throw new IllegalArgumentException();
        }
    }
}
