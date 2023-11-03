package day06;

import java.util.List;

public class VariableLightGrid {
    int width;
    int height;
    int[][] lights;

    VariableLightGrid(int width, int height) {
        this.width = width;
        this.height = height;
        this.lights = new int[height][width];
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
                lights[y][x] += 1;
            }
        }
    }

    void switchOffArea(Rect area) {
        for (int y = area.pos.y; y < area.pos.y + area.height; y++) {
            for (int x = area.pos.x; x < area.pos.x + area.width; x++) {
                lights[y][x] = lights[y][x] > 0 ? lights[y][x] - 1 : lights[y][x];
            }
        }
    }

    void toggleArea(Rect area) {
        for (int y = area.pos.y; y < area.pos.y + area.height; y++) {
            for (int x = area.pos.x; x < area.pos.x + area.width; x++) {
                lights[y][x] += 2;
            }
        }
    }

    public int brightness() {
        int brightness = 0;
        for (int[] row : lights) {
            for (int light : row) {
                brightness += light;
            }
        }
        return brightness;
    }

}
