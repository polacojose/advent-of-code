package day13;

public class Relationship {
    String from;
    String to;
    int happiness;

    Relationship(String from, String to, int happiness) {
        this.from = from;
        this.to = to;
        this.happiness = happiness;
    }

    public static Relationship parseString(String s) {
        String[] parts = s.trim().split("\\s+");
        String from = parts[0];
        String to = parts[10].replace(".", "");

        int happiness = Integer.valueOf(parts[3]);
        if (parts[2].equals("lose")) {
            happiness = -happiness;
        }

        return new Relationship(from, to, happiness);
    }
}
