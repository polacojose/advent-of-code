package day07;

public abstract class WireDefinition {

    String name;

    abstract String[] dependencies();

    abstract int resolve(int[] resolvedDeps);

    public static WireDefinition parseString(String s) {
        String[] parts = s.trim().split(" ");

        if (s.contains("AND")) {
            AndWireDefinition wd = new AndWireDefinition(parts[0], parts[2]);
            wd.name = parts[4];
            return wd;
        }

        if (s.contains("NOT")) {
            NotWireDefinition wd = new NotWireDefinition(parts[1]);
            wd.name = parts[3];
            return wd;
        }

        if (s.contains("OR")) {
            OrWireDefinition wd = new OrWireDefinition(parts[0], parts[2]);
            wd.name = parts[4];
            return wd;
        }

        if (s.contains("LSHIFT")) {
            LShiftWireDefinition wd = new LShiftWireDefinition(parts[0], parts[2]);
            wd.name = parts[4];
            return wd;
        }

        if (s.contains("RSHIFT")) {
            RShiftWireDefinition wd = new RShiftWireDefinition(parts[0], parts[2]);
            wd.name = parts[4];
            return wd;
        }

        SetWireDefinition set = new SetWireDefinition(parts[0]);
        set.name = parts[2];
        return set;
    }
}

class SetWireDefinition extends WireDefinition {

    String opA;

    SetWireDefinition(String opA) {
        this.opA = opA;
    }

    @Override
    String[] dependencies() {
        return new String[] { opA };
    }

    @Override
    int resolve(int[] resolvedDeps) {
        return resolvedDeps[0];
    }
}

class AndWireDefinition extends WireDefinition {

    String opA;
    String opB;

    AndWireDefinition(String opA, String opB) {
        this.opA = opA;
        this.opB = opB;
    }

    @Override
    String[] dependencies() {
        return new String[] { opA, opB };
    }

    @Override
    int resolve(int[] resolvedDeps) {
        return resolvedDeps[0] & resolvedDeps[1];
    }
}

class NotWireDefinition extends WireDefinition {

    String opA;

    NotWireDefinition(String opA) {
        this.opA = opA;
    }

    @Override
    String[] dependencies() {
        return new String[] { opA };
    }

    @Override
    int resolve(int[] resolvedDeps) {
        return 65535 - resolvedDeps[0];
    }
}

class OrWireDefinition extends WireDefinition {

    String opA;
    String opB;

    OrWireDefinition(String opA, String opB) {
        this.opA = opA;
        this.opB = opB;
    }

    @Override
    String[] dependencies() {
        return new String[] { opA, opB };
    }

    @Override
    int resolve(int[] resolvedDeps) {
        return resolvedDeps[0] | resolvedDeps[1];
    }
}

class LShiftWireDefinition extends WireDefinition {

    String opA;
    String opB;

    LShiftWireDefinition(String opA, String opB) {
        this.opA = opA;
        this.opB = opB;
    }

    @Override
    String[] dependencies() {
        return new String[] { opA, opB };
    }

    @Override
    int resolve(int[] resolvedDeps) {
        return resolvedDeps[0] << resolvedDeps[1];
    }
}

class RShiftWireDefinition extends WireDefinition {

    String opA;
    String opB;

    RShiftWireDefinition(String opA, String opB) {
        this.opA = opA;
        this.opB = opB;
    }

    @Override
    String[] dependencies() {
        return new String[] { opA, opB };
    }

    @Override
    int resolve(int[] resolvedDeps) {
        return resolvedDeps[0] >> resolvedDeps[1];
    }
}
