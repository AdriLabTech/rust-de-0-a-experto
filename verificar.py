#!/usr/bin/env python3
"""Comprueba que todos los ejemplos del repositorio hacen lo que dicen. Solo biblioteca estándar.

    python3 verificar.py

- Cada epNN.rs se compila con -D warnings (los avisos cuentan como error) y se ejecuta: debe terminar bien.
- Cada epNN_err.rs debe FALLAR con el código de error documentado (ep09_err compila y entra en pánico: código 101).
- vs1-java-rust-python: los programas de Rust siempre; los de Python y Java solo si están instalados.
Termina con código distinto de cero si algo no cumple. Necesita rustc (edición 2024).
"""
import re, shutil, subprocess, sys, tempfile
from pathlib import Path

RAIZ = Path(__file__).resolve().parent
TMP = Path(tempfile.mkdtemp(prefix="rust0experto_"))
fallos = []

# Código de error que debe dar cada ejemplo con fallo. None = error sin código (macro inexistente).
ERRORES = {
    "ep01_err": None, "ep02_err": "E0384", "ep03_err": "E0308", "ep04_err": "E0277", "ep05_err": "E0277",
    "ep06_err": "E0308", "ep08_err": "E0308", "ep10_err": "E0382", "ep11_err": "E0502", "ep12_err": "E0594",
    "ep13_err": "E0004", "ep14_err": "E0369", "ep17_err": "E0369",
}
ENTRADAS = {  # programas que leen de la entrada estándar
    "ep05": "5\n",
    "ep16_habitos": "add agua 8\nlist\ntotals\nquit\n",
}


def ok(nombre, detalle=""):
    print(f"  ok    {nombre} {detalle}".rstrip())


def mal(nombre, motivo):
    fallos.append(nombre)
    print(f"  FALLA {nombre}: {motivo}")


def compilar(fuente, nombre, extra=()):
    exe = TMP / nombre
    c = subprocess.run(["rustc", "--edition", "2024", "-D", "warnings", *extra, str(fuente), "-o", str(exe)],
                       capture_output=True, text=True)
    return exe, c


def ejecutar(exe, entrada=None, args=()):
    return subprocess.run([str(exe), *args], input=entrada, capture_output=True, text=True, timeout=120)


def episodios():
    print("episodios/")
    for f in sorted((RAIZ / "episodios").glob("ep*.rs")):
        n = f.stem
        exe, c = compilar(f, n)
        if n in ERRORES:
            codigos = re.findall(r"error\[(E\d+)\]", c.stderr)
            esperado = ERRORES[n]
            if c.returncode == 0:
                mal(n, "debía fallar al compilar y compila")
            elif esperado and esperado not in codigos:
                mal(n, f"esperaba {esperado}, da {codigos or 'otro error'}")
            elif esperado is None and "cannot find macro" not in c.stderr:
                mal(n, "esperaba «cannot find macro»")
            else:
                ok(n, f"falla como debe ({esperado or 'macro inexistente'})")
        elif n == "ep09_err":
            if c.returncode != 0:
                mal(n, "debía compilar")
            elif ejecutar(exe).returncode != 101:
                mal(n, "debía entrar en pánico (código 101)")
            else:
                ok(n, "compila y entra en pánico (101)")
        elif c.returncode != 0:
            mal(n, "no compila:\n" + c.stderr[:400])
        else:
            args = [str(TMP / "habitos_prueba.txt")] if n == "ep16_habitos" else []
            r = ejecutar(exe, ENTRADAS.get(n), args)
            ok(n) if r.returncode == 0 else mal(n, f"código de salida {r.returncode}")


def vs1():
    print("vs1-java-rust-python/")
    d = RAIZ / "vs1-java-rust-python"
    for f in sorted(d.glob("*.rs")):
        n = f.stem
        optimizado = ("-O",) if n in ("primos", "fact") else ()
        exe, c = compilar(f, "vs_" + n, optimizado)
        if n == "fizzbuzz_incompleto":
            ok(n, "falla como debe (E0004)") if "E0004" in c.stderr else mal(n, "esperaba E0004")
            continue
        if c.returncode != 0:
            mal(n, "no compila:\n" + c.stderr[:300]); continue
        r = ejecutar(exe, args=["30000"] if n == "primos" else [])
        if n == "primos" and not r.stdout.startswith("3245 "):
            mal(n, f"esperaba 3245 primos, da {r.stdout!r}")
        elif n == "fact" and "12963097176472289280" not in r.stdout:
            mal(n, "con -O debía dar 12963097176472289280 sin avisar")
        elif r.returncode != 0:
            mal(n, f"código de salida {r.returncode}")
        else:
            ok(n)
    if shutil.which("python3"):
        for f in sorted(d.glob("*.py")):
            r = subprocess.run([sys.executable, "-B", str(f), "30000"], capture_output=True, text=True)
            ok(f.name) if r.returncode == 0 else mal(f.name, r.stderr[:200])
    else:
        print("  (python3 no encontrado: se omiten los .py)")
    if shutil.which("javac") and shutil.which("java"):
        for f in sorted(d.glob("*.java")):
            c = subprocess.run(["javac", "-d", str(TMP / "java"), str(f)], capture_output=True, text=True)
            if c.returncode != 0:
                mal(f.name, c.stderr[:300]); continue
            r = subprocess.run(["java", "-cp", str(TMP / "java"), f.stem, "30000"], capture_output=True, text=True)
            ok(f.name) if r.returncode == 0 else mal(f.name, r.stderr[:200])
    else:
        print("  (JDK no encontrado: se omiten los .java)")


if __name__ == "__main__":
    if not shutil.which("rustc"):
        sys.exit("Falta rustc: instala Rust desde https://rustup.rs")
    print(subprocess.run(["rustc", "--version"], capture_output=True, text=True).stdout.strip())
    episodios()
    vs1()
    shutil.rmtree(TMP, ignore_errors=True)
    print(f"\n{'TODO CORRECTO' if not fallos else 'FALLOS: ' + ', '.join(fallos)}")
    sys.exit(1 if fallos else 0)
