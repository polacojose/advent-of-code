package day11;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class SantaPasswordTest {
    @Test
    void canValidatePassword() {
        assertTrue(SantaPassword.isValid("hijklmmn") == false);
        assertTrue(SantaPassword.isValid("abbceffg") == false);
        assertTrue(SantaPassword.isValid("abbcegjk") == false);
        assertTrue(SantaPassword.isValid("hxbxwxbb") == false);
        assertTrue(SantaPassword.isValid("abcdffaa"));
        assertTrue(SantaPassword.isValid("ghjaabcc"));
    }

    @Test
    void canGetNextPassword() {
        assertTrue(SantaPassword.nextPassword("abcdefgh").equals("abcdffaa"));
        assertTrue(SantaPassword.nextPassword("ghijklmn").equals("ghjaabcc"));
    }
}
