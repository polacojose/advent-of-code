/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package day03;

import java.io.IOException;
import java.nio.file.Files;
import java.nio.file.Paths;

public class App {

    public static void main(String[] args) {
        part1();
        part2();
    }

    public static void part1() {
        try {
            String ticks = Files.readString(Paths.get("input.txt"));
            int visited = Delivery.visitedHomes(ticks);
            System.out.println(String.format("Visited homes: %d", visited));
        } catch (IOException e) {
            e.printStackTrace();
        }
    }

    public static void part2() {
        try {
            String ticks = Files.readString(Paths.get("input.txt"));
            int visited = Delivery.visitedHomesWithRoboSanta(ticks);
            System.out.println(String.format("Visited homes with RoboSanta: %d", visited));
        } catch (IOException e) {
            e.printStackTrace();
        }
    }
}
