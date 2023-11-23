package day17;

import static org.junit.jupiter.api.Assertions.assertEquals;

import java.util.Set;

import org.junit.jupiter.api.Test;

class ContainerCombinationsTest {
    @Test
    void canFindCombinations() {
        Container[] containers = Container.parse(new String[] { "20", "15", "10", "5", "5" });

        Set<Set<Container>> combinations = ContainerCombinations.findCombinations(containers, 25);

        assertEquals(4, combinations.size());
    }
}
