/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package day02;

import java.io.BufferedReader;
import java.io.FileReader;

public class App {
    public static void main(String[] args) {
        part1();
        part2();
    }

    static void part1() {
        BufferedReader reader;
        try {
            reader = new BufferedReader(new FileReader("input.txt"));

            int totalWrappingPaper = 0;

            String line = reader.readLine();
            while (line != null) {
                totalWrappingPaper += GiftBox.parse(line).wrappingPaperRequired();
                line = reader.readLine();
            }

            System.out.println(String.format("Total Wrapping Paper: %d", totalWrappingPaper));

        } catch (Exception e) {
            e.printStackTrace();
        }
    }

    static void part2() {
        BufferedReader reader;
        try {
            reader = new BufferedReader(new FileReader("input.txt"));

            int totalRibbon = 0;

            String line = reader.readLine();
            while (line != null) {
                totalRibbon += GiftBox.parse(line).ribbonRequired();
                line = reader.readLine();
            }

            System.out.println(String.format("Total Ribbon: %d", totalRibbon));

        } catch (Exception e) {
            e.printStackTrace();
        }
    }
}