def es_palindromo(palabra):
    limpia = palabra.lower().replace(" ", "")
    return limpia == limpia[::-1]
print(es_palindromo("Somos o no somos"), es_palindromo("hola"))

# ---------------------------------------------------------------------------
# VS // 01 · Palindromos en Python.   Ejecutar:  python3 pal.py   -> True False
# Normaliza (minusculas y sin espacios) y compara con la cadena al reves: limpia[::-1].
