/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package day19;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.util.Set;

class TransformerTest {
    @Test void canParseTransform() {
        Transform transform = Transform.fromString("H => HO");
        assertEquals(transform.from, "H");
        assertEquals(transform.to, "HO");
        
         transform = Transform.fromString("H => OH");
        assertEquals(transform.from, "H");
        assertEquals(transform.to, "OH");
        
         transform = Transform.fromString("O => HH");
        assertEquals(transform.from, "O");
        assertEquals(transform.to, "HH");
    }
    
    @Test void canParseTransformer() throws FileNotFoundException, IOException {
        Transformer transformer = Transformer.fromStream(new FileInputStream("test-input.txt"));

        assertTrue(transformer.getTransforms().length == 3);
    }

    @Test void canTransform() throws FileNotFoundException, IOException {
        Transformer transformer = Transformer.fromStream(new FileInputStream("test-input.txt"));
        
        Set<String> transformations = transformer.transformMolecule("HOH");

        assertTrue(transformations.size() == 4);
    }

    @Test void canCountStepsToConstruction() throws FileNotFoundException, IOException {
        Transformer transformer = Transformer.fromStream(new FileInputStream("test-input2.txt"));
        
        int steps = transformer.stepsToConstruct("HOH");

        assertTrue(steps == 3);
    }
}