package day20;

public class ElfVisitor {

    final int presentsMultiplier;

    ElfVisitor(int presentsMultiplier){
        this.presentsMultiplier = presentsMultiplier;
    }

    public int countPresentsAtInfiniteVisits(int houseNumber){

        int total = 0;

        for (int i=1; i <= Math.sqrt(houseNumber); i++){
            if (houseNumber % i == 0){

                total += i * this.presentsMultiplier;

                if(houseNumber / i != i){
                    total += (houseNumber/i) * this.presentsMultiplier;
                }
            }
        }


        return total;
    }

    public int countPresentsAtFiniteVisits(int houseNumber){

        int total = 0;

        for (int i=1; i <= Math.sqrt(houseNumber); i++){
            if (houseNumber % i == 0){

                if (houseNumber <= i*50){
                    total += i * this.presentsMultiplier;
                }

                int div = houseNumber / i;
                if(div != i){
                    if(houseNumber <= div*50){
                        total += div * this.presentsMultiplier;
                    }
                }
            }
        }

        return total;
    }
}
