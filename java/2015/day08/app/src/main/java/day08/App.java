/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package day08;

import filelinesloader.FileLinesLoader;

public class App {
    public static void main(String[] args) {
        String[] lines = FileLinesLoader.getLines("input.txt");
        part1(lines);
        part2(lines);
    }

    private static void part1(String[] lines) {
        int totalDiff = 0;
        for (String line : lines) {
            totalDiff += StringEscapeCount.decodedDiff(line);
        }

        System.out.println(String.format("Total Diff: %d", totalDiff));
    }

    private static void part2(String[] lines) {
        int totalDiff = 0;
        for (String line : lines) {
            totalDiff += StringEscapeCount.encodedDiff(line);
        }

        System.out.println(String.format("Total Diff: %d", totalDiff));
    }
}
