package day13;

import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class HappinessFinder {

    Relationship[] relationships;
    Set<String> nodes = new HashSet<>();

    HappinessFinder(Relationship[] relationships) {
        for (Relationship r : relationships) {
            nodes.add(r.from);
        }
        this.relationships = relationships;
    }

    static int relationshipsMin(Relationship[] relationships) {
        int searchPosOffset = 0;
        for (Relationship r : relationships) {
            if (r.happiness < searchPosOffset) {
                searchPosOffset = r.happiness;
            }
        }
        return searchPosOffset;
    }

    int maxHappiness() {

        ArrayList<List<String>> seatings = new ArrayList<>();
        seatings.add(Arrays.asList(new String[] { nodes.toArray(new String[] {})[0] }));

        int max = 0;

        while (seatings.size() > 0) {
            List<String> seating = seatings.remove(seatings.size() - 1);

            int seatingHappiness = totalSeatingHappiness(seating);
            if (seating.size() == this.nodes.size() && seatingHappiness > max) {
                max = seatingHappiness;
            }

            for (String rem : remainingNodes(seating)) {
                List<String> newSeating = new ArrayList<>(seating);
                newSeating.add(rem);
                seatings.add(newSeating);
            }
        }

        return max;
    }

    Set<String> remainingNodes(List<String> seating) {
        Set<String> remaining = new HashSet<>();
        for (String node : this.nodes) {
            if (!seating.contains(node)) {
                remaining.add(node);
            }
        }
        return remaining;
    }

    int totalSeatingHappiness(List<String> seating) {

        if (seating.size() <= 1) {
            return 0;
        }

        int happiness = 0;

        for (int i = 0; i < seating.size() - 1; i++) {

            String a = seating.get(i);
            String b = seating.get(i + 1);
            happiness += findRelationship(a, b).happiness;
            happiness += findRelationship(b, a).happiness;
        }
        String a = seating.get(0);
        String b = seating.get(seating.size() - 1);
        happiness += findRelationship(a, b).happiness;
        happiness += findRelationship(b, a).happiness;

        return happiness;
    }

    Relationship findRelationship(String from, String to) {
        for (Relationship r : relationships) {
            if (r.from.equals(from) && r.to.equals(to)) {
                return r;
            }
        }
        return null;
    }

}
