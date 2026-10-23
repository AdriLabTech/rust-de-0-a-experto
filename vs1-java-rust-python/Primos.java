public class Primos {
    static boolean esPrimo(int numero) {
        if (numero < 2) return false;
        for (int i = 2; i < numero; i++) {
            if (numero % i == 0) return false;
        }
        return true;
    }

    public static void main(String[] args) {
        int limite = Integer.parseInt(args[0]);
        long t = System.nanoTime();
        int cuenta = 0;
        for (int n = 0; n < limite; n++) {
            if (esPrimo(n)) cuenta++;
        }
        System.out.println(cuenta + " " + (System.nanoTime() - t) / 1_000_000 + " ms");
    }
}

// ---------------------------------------------------------------------------
// VS // 01 · Contar primos por debajo de un limite.   Ejecutar:  javac Primos.java && java Primos 30000
// Misma cuenta que en Python y Rust (3245 para 30000); el tiempo cambia con la maquina y con el calentamiento de la JVM.
