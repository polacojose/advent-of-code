package day08;

import java.util.ArrayList;
import java.util.stream.Collectors;

public class StringEscapeCount {

    static int decodedDiff(String string) {
        return string.length() - decodedMemoryLength(string);
    }

    static int encodedDiff(String string) {
        return encodedMemoryLength(string) - string.length();
    }

    static int encodedMemoryLength(String s) {

        if (s.length() == 0) {
            return 0;
        }

        int count = 0;

        ArrayList<Character> chars = new ArrayList<>(s.chars().mapToObj(c -> (char) c).collect(Collectors.toList()));

        do {
            Character c = chars.remove(0);

            if ((c == '\\') || (c == '"')) {
                count += 2;
                continue;
            }

            count++;
        } while (chars.size() > 0);

        return count + 2;
    }

    static int decodedMemoryLength(String s) {

        if (s.length() == 0) {
            return 0;
        }

        int count = 0;

        ArrayList<Character> chars = new ArrayList<>(s.chars().mapToObj(c -> (char) c).collect(Collectors.toList()));

        do {
            Character c = chars.remove(0);
            if (c == '\\') {
                c = chars.remove(0);
                if (c == 'x') {
                    chars.remove(0);
                    chars.remove(0);
                    count++;
                } else {
                    count++;
                }
                continue;
            }

            count++;
        } while (chars.size() > 0);

        return count - 2;
    }
}
