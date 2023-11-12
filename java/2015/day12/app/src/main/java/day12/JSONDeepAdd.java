package day12;

import org.json.JSONArray;
import org.json.JSONObject;

public class JSONDeepAdd {

    public static int add(String jsonString) {
        return add(jsonString, null);
    }

    public static int add(String jsonString, String key) {
        try {
            return addJSONObject(new JSONObject(jsonString), key);
        } catch (Exception e) {
            return addJSONArray(new JSONArray(jsonString), key);
        }
    }

    static int addJSONObject(JSONObject jObject, String key) {
        int sum = 0;
        String[] names = JSONObject.getNames(jObject);

        if (names == null) {
            return 0;
        }

        if (key != null) {
            for (String o : names) {
                if (jObject.get(o).equals(key)) {
                    return 0;
                }
            }
        }

        for (String o : names) {

            if (o.equals(key)) {
                continue;
            }

            if (jObject.get(o) instanceof Integer) {
                sum += (Integer) jObject.get(o);
            }
            if (jObject.get(o) instanceof JSONArray) {
                sum += addJSONArray((JSONArray) jObject.get(o), key);
            }
            if (jObject.get(o) instanceof JSONObject) {
                sum += addJSONObject((JSONObject) jObject.get(o), key);
            }
        }
        return sum;
    }

    static int addJSONArray(JSONArray jArray, String key) {
        int sum = 0;

        for (Object obj : jArray) {
            if (obj instanceof Integer) {
                sum += (Integer) obj;
            }
            if (obj instanceof JSONArray) {
                sum += addJSONArray((JSONArray) obj, key);
            }
            if (obj instanceof JSONObject) {
                sum += addJSONObject((JSONObject) obj, key);
            }
        }
        return sum;
    }

}
