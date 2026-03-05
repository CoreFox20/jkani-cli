# jkani-cli 🍥

Cliente de línea de comandos para ver anime desde la terminal, usando [jkanime.net](https://jkanime.net) como fuente.

## Requisitos previos

### Linux
- `mpv` instalado
```bash
sudo apt install mpv        # Debian/Ubuntu
sudo pacman -S mpv          # Arch Linux
sudo dnf install mpv        # Fedora
```

### Windows
- [mpv para Windows](https://mpv.io/installation/) instalado y agregado al PATH

---

## Instalación

### 1. Clonar el repositorio

```bash
git clone https://github.com/CoreFox20/jkani-cli.git
cd jkani-cli
```

### 2. Instalar Rust

Si no tienes Rust instalado, instálalo con rustup:

**Linux/macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
```

**Windows:**

Descarga e instala [rustup-init.exe](https://rustup.rs)

---

## Compilar

### Linux

```bash
cargo build --release
```

El ejecutable estará en:
```
target/release/jkani-cli
```

Ejecutar directamente:
```bash
./target/release/jkani-cli
```

O instalar globalmente:
```bash
sudo cp target/release/jkani-cli /usr/local/bin/jkani
jkani
```

---

### Windows (cross-compilar desde Linux)

#### 1. Instalar el target y el linker

```bash
rustup target add x86_64-pc-windows-gnu
sudo apt install mingw-w64
```

#### 2. Compilar

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

El ejecutable estará en:
```
target/x86_64-pc-windows-gnu/release/jkani-cli.exe
```

#### 3. Compilar directamente en Windows

Si estás compilando desde Windows nativamente:

```bash
cargo build --release
```

El ejecutable estará en:
```
target\release\jkani-cli.exe
```

---

## Uso

```
jkani-cli
```

1. Escribe el nombre del anime a buscar
2. Elige de la lista de resultados
3. Elige el episodio
4. El video se abre automáticamente en `mpv`

---

## Dependencias

| Crate | Uso |
|---|---|
| `reqwest` | Peticiones HTTP |
| `scraper` | Parsear HTML |
| `inquire` | Menú interactivo en terminal |
| `colored` | Colores en terminal |
| `regex` | Extraer URLs |
| `base64` | Decodificar URLs del video |
| `tokio` | Runtime async |
