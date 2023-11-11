package day09;

import java.util.HashMap;
import java.util.HashSet;
import java.util.List;
import java.util.Map;
import java.util.Set;
import java.util.ArrayList;
import java.util.Arrays;

public class PathFinder {

    Set<String> nodes;
    Map<String, Integer> paths = new HashMap<>();

    PathFinder(PathLeg[] paths) {
        nodes = new HashSet<>();
        for (PathLeg p : paths) {
            nodes.add(p.from);
            nodes.add(p.to);
            this.paths.put(PathKey(p.from, p.to), p.distance);
            this.paths.put(PathKey(p.to, p.from), p.distance);
        }
    }

    String PathKey(String from, String to) {
        return String.format("%s-%s", from, to);
    }

    public int shortestDistanceThroughAllNodes() {
        List<List<String>> paths = new ArrayList<>();
        for (String n : this.nodes) {
            paths.add(new ArrayList<>(Arrays.asList(new String[] { n })));
        }

        int min_distance = Integer.MAX_VALUE;

        do {
            List<String> path = paths.remove(0);
            int length = getPathLength(path);

            if (path.size() == nodes.size()) {

                min_distance = Math.min(min_distance, length);

                continue;
            }

            if (length >= min_distance) {
                continue;
            }

            Set<String> r_nodes = remainingNodes(path);
            for (String n : r_nodes) {
                List<String> n_path = new ArrayList<>(path);
                n_path.add(n);
                paths.add(n_path);
            }

        } while (paths.size() > 0);

        return min_distance;
    }

    public int longestDistanceThroughAllNodes() {
        List<List<String>> paths = new ArrayList<>();
        for (String n : this.nodes) {
            paths.add(new ArrayList<>(Arrays.asList(new String[] { n })));
        }

        int max_distance = 0;

        do {
            List<String> path = paths.remove(0);
            int length = getPathLength(path);

            if (path.size() == nodes.size()) {
                max_distance = Math.max(max_distance, length);
                continue;
            }

            Set<String> r_nodes = remainingNodes(path);
            for (String n : r_nodes) {
                List<String> n_path = new ArrayList<>(path);
                n_path.add(n);
                paths.add(n_path);
            }

        } while (paths.size() > 0);

        return max_distance;
    }

    public Set<String> remainingNodes(List<String> path) {
        Set<String> nodes = new HashSet<>(this.nodes);
        nodes.removeAll(new HashSet<String>(path));
        return nodes;
    }

    public int getPathLength(List<String> nodes) {
        int pathLength = 0;
        for (int i = 1; i < nodes.size(); i++) {
            pathLength += paths.get(PathKey(nodes.get(i - 1), nodes.get(i)));
        }
        return pathLength;
    }
}
