package day07;

import java.util.HashMap;
import java.util.Map;

public class Circuit {

    Map<String, WireDefinition> wireMap;
    Map<String, Integer> resolvedWires;

    Circuit(WireDefinition[] wireDefinitions) {
        this.wireMap = new HashMap<>();
        for (WireDefinition wd : wireDefinitions) {
            this.wireMap.put(wd.name, wd);
        }
        this.resolvedWires = new HashMap<>();
    }

    int resolveWireValue(String wireName) {
        if (!wireMap.containsKey(wireName)) {
            System.out.println(wireName);
            throw new IllegalArgumentException();
        }

        if (resolvedWires.containsKey(wireName)) {
            return resolvedWires.get(wireName);
        }

        WireDefinition wd = wireMap.get(wireName);
        String[] deps = wd.dependencies();
        int[] resolvedDeps = new int[deps.length];
        for (int i = 0; i < deps.length; i++) {
            try {
                resolvedDeps[i] = Integer.valueOf(deps[i]);
            } catch (Exception e) {
                resolvedDeps[i] = resolveWireValue(deps[i]);
                this.resolvedWires.put(deps[i], resolvedDeps[i]);
            }
        }
        return wd.resolve(resolvedDeps);
    }
}
