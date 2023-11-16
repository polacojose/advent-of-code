
package day14;

public class Reindeer {

    String name;
    int speed;
    int flyTime;
    int restTime;

    Reindeer(String name, int speed, int flyTime, int restTime) {
        this.name = name;
        this.speed = speed;
        this.flyTime = flyTime;
        this.restTime = restTime;
    }

    public static Reindeer parseString(String s) {
        String[] parts = s.trim().split("\\s+");

        String name = parts[0];
        int speed = Integer.valueOf(parts[3]);
        int flyTime = Integer.valueOf(parts[6]);
        int restTime = Integer.valueOf(parts[13]);

        return new Reindeer(name, speed, flyTime, restTime);
    }

    int distance(int seconds) {

        int distance = 0;

        int cycleTime = restTime + flyTime;

        if (seconds >= cycleTime) {
            int dashes = seconds / cycleTime;
            distance += speed * flyTime * dashes;
            seconds -= dashes * cycleTime;
        }
        distance += speed * Math.min(flyTime, seconds);

        return distance;
    }

    @Override
    public boolean equals(Object obj) {

        if (obj == null) {
            return false;
        }

        if (!(obj instanceof Reindeer)) {
            return false;
        }

        Reindeer o = (Reindeer) obj;

        return this.name.equals(o.name) && this.speed == o.speed && this.flyTime == o.flyTime
                && this.restTime == o.restTime;
    }
}
