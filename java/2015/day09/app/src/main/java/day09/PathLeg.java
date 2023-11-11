package day09;

public class PathLeg {

    String from;
    String to;
    int distance;

    PathLeg(String from, String to, int distance) {
        this.from = from;
        this.to = to;
        this.distance = distance;
    }

    public static PathLeg[] parsePathLegString(String path) {
        String[] pathParts = path.trim().split("\\s+");
        int distance = Integer.valueOf(pathParts[4]);

        PathLeg[] paths = new PathLeg[2];
        paths[0] = new PathLeg(pathParts[0], pathParts[2], distance);
        paths[1] = new PathLeg(pathParts[2], pathParts[0], distance);

        return paths;
    }

    @Override
    public boolean equals(Object obj) {
        if (obj == null) {
            return false;
        }
        PathLeg o = (PathLeg) obj;
        return this.from.equals(o.from) &&
                this.to.equals(o.to) &&
                this.distance == o.distance;
    }

    @Override
    public int hashCode() {
        return new String[] { from, to }.hashCode() * Integer.hashCode(distance);
    }
}
