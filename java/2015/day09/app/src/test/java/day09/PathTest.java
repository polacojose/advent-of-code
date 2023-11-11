package day09;

import org.junit.jupiter.api.Test;
import static org.junit.jupiter.api.Assertions.*;

class PathTest {
    @Test
    void canParsePaths() {
        String pathString = "London to Dublin = 464";
        PathLeg[] parsedPaths = PathLeg.parsePathLegString(pathString);

        PathLeg[] expectedPaths = new PathLeg[] {
                new PathLeg("London", "Dublin", 464),
                new PathLeg("Dublin", "London", 464),
        };

        for (int i = 0; i < 2; i++) {
            assertTrue(expectedPaths[i].equals(parsedPaths[i]));
        }
    }
}
