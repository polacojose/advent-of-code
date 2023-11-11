package day10;

public class LookAndSay {
    public static String lookAndSay(String s) {

        if (s.isBlank()) {
            throw new IllegalArgumentException();
        }

        char[] chars = s.toCharArray();
        char lastChar = chars[0];
        int numChar = 1;

        StringBuilder sb = new StringBuilder();

        for (int i = 1; i < chars.length; i++) {
            if (chars[i] != lastChar) {
                sb.append(numChar);
                sb.append(lastChar);
                lastChar = chars[i];
                numChar = 0;
            }
            numChar++;
        }
        sb.append(numChar);
        sb.append(lastChar);

        return sb.toString();
    }
}
