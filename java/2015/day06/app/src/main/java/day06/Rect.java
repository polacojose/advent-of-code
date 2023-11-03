package day06;

public class Rect {
    Point pos;
    int width;
    int height;

    Rect(int posX, int posY, int width, int height) {
        this.pos = new Point(posX, posY);
        this.width = width;
        this.height = height;
    }

    Rect(Point pointA, Point pointB) {
        this.pos = pointA;
        this.width = pointB.x - pointA.x + 1;
        this.height = pointB.y - pointA.y + 1;
    }
}
