package day11;

public class SantaPassword {

    static boolean hasThreeConsecutiveLetters(String password) {
        for (int i = 0; i < password.length() - 2; i++) {
            if (password.charAt(i) + 1 == password.charAt(i + 1)
                    && password.charAt(i + 1) + 1 == password.charAt(i + 2)) {
                return true;
            }
        }

        return false;
    }

    static boolean hasInvalidCharacters(String password) {
        return password.contains("i") || password.contains("o") || password.contains("l");
    }

    static boolean hasTwoPairs(String password) {
        int pairs = 0;
        for (int i = 0; i < password.length() - 1; i++) {
            if (password.charAt(i) == password.charAt(i + 1)) {
                pairs++;
                i++;
            }
            if (pairs >= 2) {
                return true;
            }
        }
        return false;
    }

    public static boolean isValid(String password) {
        return hasThreeConsecutiveLetters(password) && !hasInvalidCharacters(password) && hasTwoPairs(password);
    }

    static String incrementPassword(String password) {
        char[] chars = password.toCharArray();
        boolean carry = true;
        for (int i = chars.length - 1; i >= 0; i--) {
            if (carry) {
                carry = false;
                if (chars[i] == 'z') {
                    chars[i] = 'a';
                    carry = true;
                } else {
                    chars[i]++;
                    break;
                }
            }
        }
        return new String(chars);
    }

    public static String nextPassword(String startPassword) {
        String password = incrementPassword(startPassword);
        while (!isValid(password)) {
            password = incrementPassword(password);
        }

        return password;
    }
}
