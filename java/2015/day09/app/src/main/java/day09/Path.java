package day09;

public class Path {

    PathLeg[] pathLegs;
    int length;

    Path(PathLeg[] pathLegs) {
        this.pathLegs = pathLegs;
        this.length = Path.addLengths(pathLegs);
    }

    static int addLengths(PathLeg[] pathLegs) {
        int length = 0;
        for (PathLeg pl : pathLegs) {
            length += pl.distance;
        }
        return length;
    }

    public int length() {
        return length;
    }
}
