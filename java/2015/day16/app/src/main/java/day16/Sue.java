package day16;

import java.util.HashMap;
import java.util.Map;

public class Sue {
    public final int id;
    public final Map<String, Integer> attributes;

    public Sue(int id, Map<String, Integer> attributes) {
        this.id = id;
        this.attributes = attributes;
    }

    public static Sue parseString(String s) {
        String[] parts = s.trim().replace(":", "").replace(",", "").split("\\s+");

        int id = Integer.valueOf(parts[1]);

        Map<String, Integer> attributes = new HashMap<>();
        attributes.put(parts[2], Integer.valueOf(parts[3]));
        attributes.put(parts[4], Integer.valueOf(parts[5]));
        attributes.put(parts[6], Integer.valueOf(parts[7]));

        return new Sue(id, attributes);
    }

    @Override
    public boolean equals(Object obj) {
        if (obj == null) {
            return false;
        }

        if (!this.getClass().equals(obj.getClass())) {
            return false;
        }

        Sue other = (Sue) obj;
        return this.id == other.id && this.attributes.equals(other.attributes);
    }
}
