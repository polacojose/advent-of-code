package day07;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class CircuitTest {
    @Test
    void canResolveOneWireCircuit() {
        WireDefinition[] wds = new WireDefinition[] { WireDefinition.parseString("123 -> x") };
        Circuit c = new Circuit(wds);
        assertTrue(c.resolveWireValue("x") == 123);
    }

    @Test
    void canResolveTwoWireCircuit() {
        WireDefinition[] wds = new WireDefinition[] { WireDefinition.parseString("NOT x -> z"),
                WireDefinition.parseString("123 -> x") };
        Circuit c = new Circuit(wds);
        assertTrue(c.resolveWireValue("x") == 123);
        assertTrue(c.resolveWireValue("z") == 65412);

        wds = new WireDefinition[] { WireDefinition.parseString("x -> z"),
                WireDefinition.parseString("123 -> x") };
        c = new Circuit(wds);
        assertTrue(c.resolveWireValue("x") == 123);
        assertTrue(c.resolveWireValue("z") == 123);
    }

    @Test
    void canResolveThreeWireCircuit() {
        WireDefinition[] wds = new WireDefinition[] {
                WireDefinition.parseString("NOT x -> z"),
                WireDefinition.parseString("123 -> x"),
                WireDefinition.parseString("x AND z -> y"),
        };
        Circuit c = new Circuit(wds);
        assertTrue(c.resolveWireValue("x") == 123);
        assertTrue(c.resolveWireValue("z") == 65412);
        assertTrue(c.resolveWireValue("y") == 0);
    }
}
