# Graficas-diorama
Un raytracer en Rust que renderiza escenas tipo Minecraft con iluminación realista, texturas, reflejos y transparencias.
Link de video: https://uvggt-my.sharepoint.com/:v:/g/personal/men23778_uvg_edu_gt/EUnA-XjBL2VAlweS-e4U7pABR1txFZ_0Uqsa7ncZWyNGiA?e=kcGdoA&nav=eyJyZWZlcnJhbEluZm8iOnsicmVmZXJyYWxBcHAiOiJTdHJlYW1XZWJBcHAiLCJyZWZlcnJhbFZpZXciOiJTaGFyZURpYWxvZy1MaW5rIiwicmVmZXJyYWxBcHBQbGF0Zm9ybSI6IldlYiIsInJlZmVycmFsTW9kZSI6InZpZXcifX0%3D 

<img width="910" height="731" alt="image" src="https://github.com/user-attachments/assets/acd00d7e-505b-4f10-a5f6-bdd749e64d2b" />



Para hacer el modelo me basé n una construcción hecha por mi en minecraft :D

<img width="943" height="777" alt="image" src="https://github.com/user-attachments/assets/ac3c1734-f442-4607-a3ef-a917a525b64c" />



Pequeños avances

<img width="661" height="566" alt="image" src="https://github.com/user-attachments/assets/74840427-b5a5-4791-8579-c10177dae183" />



## ✨ Características
- **Raytracing en tiempo real** con paralelización usando Rayon
- **Sistema de materiales completo**: difuso, especular, reflexión, refracción
- **Múltiples fuentes de luz** con sombras dinámicas
- **Texturas por cara de cubo** (top, bottom, sides)
- **Materiales emisivos** (bloques que emiten luz como lámparas de redstone)
- **Skybox** con 6 texturas
- **Cámara orbital interactiva** con zoom
- **Rotación de cubos** en los ejes X e Y

## 🎯 Materiales Incluidos

- 🧱 **Bloques de construcción**: Ladrillos, madera, piedra
- 🌍 **Terreno**: Tierra, arena, tierra con pasto
- 💎 **Minerales**: Diamante, hierro, granito, diorita
- 💧 **Agua**: Con transparencia,  refracción
- 🌸 **Decoración**: Flores (azalea), librerías, calabazas
- 🔥 **Emisivos**: lámpara de redstone

## 🚀 Instalación

### Prerequisitos

- Rust 1.70+ (instala desde [rustup.rs](https://rustup.rs/))
- Dependencias del sistema para raylib:
  

### Compilar y Ejecutar

```bash
# Clonar el repositorio
git clone https://github.com/tuusuario/minecraft-raytracer.git

# Compilar en modo release (más rápido)
cargo build --release

# Ejecutar
cargo run --release
```

## 🎮 Controles

| Tecla | Acción |
|-------|--------|
| `A` / `D` | Rotar cámara horizontalmente |
| `W` / `S` | Rotar cámara verticalmente |
| `↑` / `↓` | Acercar / Alejar zoom |


## 📁 Estructura del Proyecto

```
minecraft-raytracer/
├── src/
│   ├── main.rs              # Loop principal y renderizado
│   ├── camera.rs            # Sistema de cámara orbital
│   ├── cube.rs              # Geometría de cubos con rotación
│   ├── framebuffer.rs       # Buffer de imagen
│   ├── light.rs             # Sistema de iluminación
│   ├── material.rs          # Definición de materiales
│   ├── materials.rs         # Biblioteca de materiales
│   ├── ray_intersect.rs     # Trait para intersección de rayos
│   ├── scene.rs             # Gestión de escena
│   ├── skybox.rs            # Sistema de skybox
│   └── textures.rs          # Carga y muestreo de texturas
├── assets/                  # Texturas de Minecraft
│   ├── brick.png
│   ├── diamond_ore.png
│   ├── water_flow.png
│   └── ...
└── Cargo.toml
```

