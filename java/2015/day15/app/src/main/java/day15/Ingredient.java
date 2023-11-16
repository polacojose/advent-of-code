package day15;

public class Ingredient {

    public String name;
    public int capacity;
    public int durability;
    public int flavor;
    public int texture;
    public int calories;

    Ingredient(String name, int capacity, int durability, int flavor, int texture, int calories) {
        this.name = name;
        this.capacity = capacity;
        this.durability = durability;
        this.flavor = flavor;
        this.texture = texture;
        this.calories = calories;
    }

    public static Ingredient parseString(String line) {
        String[] parts = line.trim().replace(",", "").split(" ");

        String name = parts[0].substring(0, parts[0].length() - 1);
        int capacity = Integer.parseInt(parts[2]);
        int durability = Integer.parseInt(parts[4]);
        int flavor = Integer.parseInt(parts[6]);
        int texture = Integer.parseInt(parts[8]);
        int calories = Integer.parseInt(parts[10]);

        return new Ingredient(name, capacity, durability, flavor, texture, calories);
    }

    @Override
    public boolean equals(Object obj) {
        if (obj == null) {
            return false;
        }

        if (getClass() != obj.getClass()) {
            return false;
        }

        Ingredient other = (Ingredient) obj;
        return this.name.equals(other.name) && this.capacity == other.capacity && this.durability == other.durability
                && this.flavor == other.flavor && this.texture == other.texture && this.calories == other.calories;
    }
}
