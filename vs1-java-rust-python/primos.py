import sys, time

def es_primo(numero):
    if numero < 2:
        return False
    for i in range(2, numero):
        if numero % i == 0:
            return False
    return True

limite = int(sys.argv[1])
t = time.perf_counter()
cuenta = sum(1 for n in range(limite) if es_primo(n))
print(cuenta, f"{(time.perf_counter() - t) * 1000:.0f} ms")

# ---------------------------------------------------------------------------
# VS // 01 · Contar primos por debajo de un limite (algoritmo ingenuo, como en clase).
# Ejecutar:  python3 primos.py 30000      -> "3245 <milisegundos> ms"
# Los tres lenguajes dan la misma cuenta (3245); el tiempo depende de la maquina: en el video se midio 3 veces y se
# tomo la mediana. Es un solo algoritmo y un solo tamano: no mide memoria ni arranque.
