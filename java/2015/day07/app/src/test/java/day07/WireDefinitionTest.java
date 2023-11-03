package day07;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class WireTest {
    @Test
    void canParseSet() {
        WireDefinition set = WireDefinition.parseString("123 -> x");
        assertTrue(set.dependencies()[0].equals("123"));
        assertTrue(set.name.equals("x"));
        assertTrue(set.resolve(new int[] { 123 }) == 123);
    }

    void canParseAnd() {
        WireDefinition and = WireDefinition.parseString("x AND y -> z");
        assertTrue(and.dependencies() == new String[] { "x", "y" });
        assertTrue(and.name.equals("z"));
        assertTrue(and.resolve(new int[] { 2, 1 }) == 0);
    }

    void canParseNot() {
        WireDefinition not = WireDefinition.parseString("NOT e -> f");
        assertTrue(not.dependencies() == new String[] { "e" });
        assertTrue(not.name.equals("f"));
        assertTrue(not.resolve(new int[] { 1 }) == 65534);
    }

    void canParseOr() {
        WireDefinition or = WireDefinition.parseString("x OR y -> e");
        assertTrue(or.dependencies() == new String[] { "x", "y" });
        assertTrue(or.name.equals("e"));
        assertTrue(or.resolve(new int[] { 2, 1 }) == 3);
    }

    void canParseLShift() {
        WireDefinition lshift = WireDefinition.parseString("p LSHIFT 2 -> q");
        assertTrue(lshift.dependencies() == new String[] { "p" });
        assertTrue(lshift.name.equals("q"));
    }

    void canParseRShift() {
        WireDefinition rshift = WireDefinition.parseString("p RSHIFT 2 -> q");
        assertTrue(rshift.dependencies() == new String[] { "p" });
        assertTrue(rshift.name.equals("q"));
    }
}
