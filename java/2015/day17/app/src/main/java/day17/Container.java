package day17;

import java.util.Arrays;

public class Container {

    public final int id;
    public final int volume;

    public Container(int id, int volume) {
        this.id = id;
        this.volume = volume;
    }

    public static Container[] parse(String[] lines) {
        Container[] containers = new Container[lines.length];
        for (int i = 0; i < lines.length; i++) {
            containers[i] = new Container(i, Integer.parseInt(lines[i].trim()));
        }
        return containers;
    }

    @Override
    public boolean equals(Object obj) {
        if (obj == null) {
            return false;
        }

        if (obj == this) {
            return true;
        }

        if (obj instanceof Container) {
            Container other = (Container) obj;
            return this.id == other.id && this.volume == other.volume;
        }

        return false;
    }

    @Override
    public int hashCode() {
        return Arrays.hashCode(new int[] { id, volume });
    }
}
