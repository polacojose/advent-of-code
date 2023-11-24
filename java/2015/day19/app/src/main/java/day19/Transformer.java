package day19;

import java.io.BufferedReader;
import java.io.IOException;
import java.io.InputStream;
import java.io.InputStreamReader;
import java.util.ArrayList;
import java.util.Arrays;
import java.util.HashSet;
import java.util.List;
import java.util.Set;

class Transform {
    public final String from;
    public final String to;

    Transform(String from, String to){
        this.from = from;
        this.to = to;
    }

    static Transform fromString(String s){
        String[] parts = s.trim().split("\s+=>\s+");
        return new Transform(parts[0], parts[1]);
    }
}

public class Transformer {

    private final Transform[] transforms;

    Transformer(Transform[] transforms){

        Arrays.sort(transforms, (a,b)->{
            return b.to.length() - a.to.length();
        });

        this.transforms = transforms;
    }

    static Transformer fromStream(InputStream input) throws IOException{
        BufferedReader br = new BufferedReader(new InputStreamReader(input));

        List<Transform> transforms = new ArrayList<>();

        String line = br.readLine();
        while (line != null){
            transforms.add(Transform.fromString(line));
            line = br.readLine();
        }

        return new Transformer(transforms.toArray(new Transform[]{}));
    }

     Set<String> transformMolecule(String s){

        Set<String> stringTransformations = new HashSet<>();

        for (Transform t: this.transforms){

            int searchIndex = 0;
            
            int foundIndex = s.indexOf(t.from, searchIndex);
            while (foundIndex >= 0){

                StringBuilder sb = new StringBuilder();

                if(foundIndex > 0){
                    sb.append(s.substring(0, foundIndex));
                }

                sb.append(t.to);

                if(foundIndex + t.from.length() < s.length()){
                    sb.append(s.substring(foundIndex + t.from.length()));
                }

                stringTransformations.add(sb.toString());

                searchIndex = foundIndex + 1;
                foundIndex = s.indexOf(t.from, searchIndex);
            }
        }

        return stringTransformations;
    }

    int stepsToConstruct(String target){
        int steps = 0;

        while (!target.equals("e")){
            for (Transform t: this.transforms){
                if (target.contains(t.to)){
                    target = target.replaceFirst(t.to, t.from);
                    steps++;
                    break;
                }
            }
        }

        return steps;
    }

    Transform[] getTransforms(){
        return this.transforms;
    }
}

















