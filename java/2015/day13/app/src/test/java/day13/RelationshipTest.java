package day13;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class RelationshipTest {

    @Test
    void canParseHappiness() {
        Relationship r = Relationship.parseString("Alice would gain 54 happiness units by sitting next to Bob.");
        assertTrue(r.from.equals("Alice"));
        assertTrue(r.to.equals("Bob"));
        assertTrue(r.happiness == 54);

        r = Relationship.parseString("Bob would lose 7 happiness units by sitting next to Carol.");
        assertTrue(r.from.equals("Bob"));
        assertTrue(r.to.equals("Carol"));
        assertTrue(r.happiness == -7);
    }

}
