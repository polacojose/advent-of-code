/*
 * This Java source file was generated by the Gradle 'init' task.
 */
package day19;

import java.io.FileInputStream;
import java.io.FileNotFoundException;
import java.io.IOException;
import java.util.Set;

public class App {

    static String startMolecule = "CRnSiRnCaPTiMgYCaPTiRnFArSiThFArCaSiThSiThPBCaCaSiRnSiRnTiTiMgArPBCaPMgYPTiRnFArFArCaSiRnBPMgArPRnCaPTiRnFArCaSiThCaCaFArPBCaCaPTiTiRnFArCaSiRnSiAlYSiThRnFArArCaSiRnBFArCaCaSiRnSiThCaCaCaFYCaPTiBCaSiThCaSiThPMgArSiRnCaPBFYCaCaFArCaCaCaCaSiThCaSiRnPRnFArPBSiThPRnFArSiRnMgArCaFYFArCaSiRnSiAlArTiTiTiTiTiTiTiRnPMgArPTiTiTiBSiRnSiAlArTiTiRnPMgArCaFYBPBPTiRnSiRnMgArSiThCaFArCaSiThFArPRnFArCaSiRnTiBSiThSiRnSiAlYCaFArPRnFArSiThCaFArCaCaSiThCaCaCaSiRnPRnCaFArFYPMgArCaPBCaPBSiRnFYPBCaFArCaSiAl";

    public static void main(String[] args) throws FileNotFoundException, IOException {
        Transformer transformer = Transformer.fromStream(new FileInputStream("input.txt"));

        part1(transformer);
        part2(transformer);
    }

    private static void part1(Transformer transformer) {
        Set<String> transformations = transformer.transformMolecule(startMolecule);

        System.out.println(String.format("Transformations: %d", transformations.size()));
    }

    private static void part2(Transformer transformer) {
        int steps = transformer.stepsToConstruct(startMolecule);
        System.out.println(String.format("Transformations: %d", steps));
    }
}