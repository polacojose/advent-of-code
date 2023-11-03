package day06;

public class Point {
    int x;
    int y;

    Point(int x, int y) {
        this.x = x;
        this.y = y;
    }

    public static Point parsePoint(String s) {
        String[] pointParts = s.trim().split(",");
        return new Point(Integer.parseInt(pointParts[0].trim()), Integer.parseInt(pointParts[1].trim()));
    }
}
