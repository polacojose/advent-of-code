package day03;

import java.util.Arrays;

public class Vector2 {
    public int x;
    public int y;

    Vector2(int x, int y) {
        this.x = x;
        this.y = y;
    }

    public Vector2 up() {
        this.y--;
        return this;
    }

    public Vector2 down() {
        this.y++;
        return this;
    }

    public Vector2 left() {
        this.x--;
        return this;
    }

    public Vector2 right() {
        this.x++;
        return this;
    }

    public Vector2 copy() {
        return new Vector2(x, y);
    }

    @Override
    public boolean equals(Object obj) {
        if (obj == null) {
            return false;
        }

        if (obj == this) {
            return true;
        }

        Vector2 other = (Vector2) obj;
        return other.x == this.x && other.y == this.y;
    }

    @Override
    public int hashCode() {
        return Arrays.hashCode(new int[] { x, y });
    }
}
