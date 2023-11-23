package day17;

import java.util.ArrayList;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

public class ContainerCombinations {

    public static Set<Set<Container>> findCombinations(Container[] containers, int targetVolume) {

        List<List<Container>> openLists = new ArrayList<>();
        for (Container c : containers) {
            ArrayList<Container> set = new ArrayList<Container>();
            set.add(c);
            openLists.add(set);
        }

        Set<Set<Container>> targetCombinations = new HashSet<>();

        while (!openLists.isEmpty()) {
            List<Container> currentList = openLists.remove(openLists.size() - 1);

            int size = sumList(currentList);
            if (size == targetVolume) {
                Set<Container> currentListSetCopy = new HashSet<>(currentList);
                targetCombinations.add(currentListSetCopy);
                continue;
            }

            int containersEndIndex = 0;
            for (int i = 0; i < containers.length; i++) {
                if (containers[i].equals(currentList.get(currentList.size() - 1))) {
                    containersEndIndex = i;
                    break;
                }
            }

            for (int i = containersEndIndex + 1; i < containers.length; i++) {
                Container c = containers[i];

                if (size + c.volume > targetVolume) {
                    continue;
                }

                List<Container> newSet = new ArrayList<>(currentList);
                newSet.add(c);
                openLists.add(newSet);
            }
        }

        return targetCombinations;
    }

    static int sumList(List<Container> containers) {
        int sum = 0;
        for (Container c : containers) {
            sum += c.volume;
        }
        return sum;
    }

}
