package day06;

import java.util.ArrayList;
import java.util.List;

public class Instruction {

    InstructionType type;
    Rect area;

    public Instruction(InstructionType type, Rect area) {
        this.type = type;
        this.area = area;
    }

    public static List<Instruction> parseString(String s) {
        List<Instruction> insts = new ArrayList<>();
        String[] lines = s.trim().split("\n");
        for (String line : lines) {
            line = line.trim();
            String[] line_parts = line.split("\\s+");

            InstructionType instType;
            Point pointA;
            Point pointB;
            switch (line_parts[1]) {
                case "on":
                    instType = InstructionType.On;
                    pointA = Point.parsePoint(line_parts[2]);
                    pointB = Point.parsePoint(line_parts[4]);
                    break;
                case "off":
                    instType = InstructionType.Off;
                    pointA = Point.parsePoint(line_parts[2]);
                    pointB = Point.parsePoint(line_parts[4]);
                    break;
                default:
                    instType = InstructionType.Toggle;
                    pointA = Point.parsePoint(line_parts[1]);
                    pointB = Point.parsePoint(line_parts[3]);
            }
            insts.add(new Instruction(instType, new Rect(pointA, pointB)));
        }
        return insts;
    }
}
