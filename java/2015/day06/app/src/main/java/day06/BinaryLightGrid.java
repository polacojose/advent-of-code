package day06;

import java.util.List;

public class BinaryLightGrid {
    int width;
    int height;
    boolean[][] lights;

    BinaryLightGrid(int width, int height) {
        this.width = width;
        this.height = height;
        this.lights = new boolean[height][width];
    }

    public void executeInstructions(List<Instruction> instructions) {
        for (Instruction inst : instructions) {
            switch (inst.type) {
                case On:
                    switchOnArea(inst.area);
                    break;
                case Off:
                    switchOffArea(inst.area);
                    break;
                case Toggle:
                    toggleArea(inst.area);
                    break;
            }
        }
    }

    void switchOnArea(Rect area) {
        for (int y = area.pos.y; y < area.pos.y + area.height; y++) {
            for (int x = area.pos.x; x < area.pos.x + area.width; x++) {
                lights[y][x] = true;
            }
        }
    }

    void switchOffArea(Rect area) {
        for (int y = area.pos.y; y < area.pos.y + area.height; y++) {
            for (int x = area.pos.x; x < area.pos.x + area.width; x++) {
                lights[y][x] = false;
            }
        }
    }

    void toggleArea(Rect area) {
        for (int y = area.pos.y; y < area.pos.y + area.height; y++) {
            for (int x = area.pos.x; x < area.pos.x + area.width; x++) {
                lights[y][x] = !lights[y][x];
            }
        }
    }

    public int lightsOn() {
        int on = 0;
        for (boolean[] row : lights) {
            for (boolean light : row) {
                if (light) {
                    on++;
                }
            }
        }
        return on;
    }

}
