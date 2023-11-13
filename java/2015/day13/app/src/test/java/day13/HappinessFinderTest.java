package day13;

import org.junit.jupiter.api.Test;

import filelinesloader.FileLinesLoader;

import static org.junit.jupiter.api.Assertions.*;

class HappinessFinderTest {

    Relationship[] testRelationships() {
        String[] lines = FileLinesLoader.getLines("test-input.txt");
        Relationship[] relationships = new Relationship[lines.length];
        for (int i = 0; i < lines.length; i++) {
            relationships[i] = Relationship.parseString(lines[i]);
        }
        return relationships;
    }

    @Test
    void canCalculateMaxHappiness() {
        Relationship[] relationships = testRelationships();
        HappinessFinder hf = new HappinessFinder(relationships);
        assertTrue(hf.maxHappiness() == 330);
    }
}
