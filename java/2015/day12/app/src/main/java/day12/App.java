/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package day12;

import java.nio.file.Files;
import java.nio.file.Paths;

public class App {
    public static void main(String[] args) {
        try {
            String input = Files.readString(Paths.get("input.txt"));
            part1(input);
            part2(input);
        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    private static void part1(String input) {
        int sum = JSONDeepAdd.add(input);
        System.out.println(String.format("Sum: %d", sum));
    }

    private static void part2(String input) {
        int sum = JSONDeepAdd.add(input, "red");
        System.out.println(String.format("Sum: %d", sum));
    }
}