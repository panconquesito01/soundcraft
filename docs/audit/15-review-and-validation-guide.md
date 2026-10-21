# SoundCraft — Guía paso a paso de revisión y validación

**Fecha de corte:** 2026-10-17
**Rama:** `feature/miscellaneous-adjustments-panconquesito`
**Base:** `origin/main`
**Documento principal:** `docs/audit/13-improvement-backlog.md`

Esta guía permite revisar el plan, el código y el comportamiento sin asumir que una función está terminada porque exista en la interfaz. Los pasos distinguen validación automatizada, inspección de código y revisión manual.

## 1. Preparar el entorno

Abrir PowerShell en la raíz del repositorio:

```powershell
Set-Location C:\Desarrollo\Colaborado\Soundcraft
git branch --show-current
git status --short
git rev-list --left-right --count HEAD...origin/main
```

Resultado esperado:

- La rama es `feature/miscellaneous-adjustments-panconquesito`.
- La comparación con `origin/main` muestra `0  0` mientras no haya nuevos commits remotos.
- Los archivos modificados y nuevos siguen presentes; no deben desaparecer durante la revisión.

Crear una referencia legible de todos los cambios:

```powershell
git diff --stat
git diff -- ROADMAP.md docs/gaps.md docs/roadmap.md
git diff -- crates/engine/src crates/midi/src crates/ui-egui/src
```

Para revisar un solo archivo en VS Code:

```powershell
code --diff (git rev-parse HEAD):crates/ui-egui/src/midi_editor.rs crates/ui-egui/src/midi_editor.rs
```

Si esa forma de `code --diff` no es admitida por la instalación local, usar el panel **Source Control** de VS Code y abrir cada archivo modificado.

## 2. Leer el plan en el orden correcto

1. Abrir `docs/audit/13-improvement-backlog.md`.
2. Leer **Estado de ejecución** para separar completado, parcial y pendiente.
3. Leer **Línea base verificada** para conocer qué se ejecutó realmente.
4. Revisar **Dependencias del plan** antes de cambiar prioridades.
5. Revisar cada iniciativa `SC-001` a `SC-026`.
6. Para cada iniciativa comprobar los campos: evidencia, impacto, solución, archivos, prioridad, riesgos, dependencias, aceptación y pruebas.
7. Terminar con **Primeras 10 acciones** y **Definition of Done global**.

En VS Code se puede localizar una iniciativa con `Ctrl+F`, por ejemplo `SC-013`.

## 3. Ejecutar las puertas de calidad

La comprobación integral recomendada es:

```powershell
cargo xtask ci
```

Debe finalizar sin errores. Este comando comprueba formato, Clippy, pruebas, activos, capas, inventario de capacidades y WASM según la configuración del repositorio.

Si se desea aislar un fallo, ejecutar en este orden:

```powershell
cargo fmt --all -- --check
cargo check --workspace --locked --offline
cargo clippy --workspace --all-targets --locked --offline -- -D warnings
cargo test -j 2 --workspace -- --test-threads=1
cargo xtask assets
cargo xtask layers
cargo xtask capabilities
cargo xtask wasm
```

Reglas de interpretación:

- Un comando con código de salida distinto de cero no está aprobado.
- Una prueba `ignored` no cuenta como ejecutada.
- Un bloqueo por permisos del sandbox no demuestra un fallo de SoundCraft; repetir fuera del sandbox y registrar ambos resultados.
- No usar `cargo fmt` sin `--check` durante una revisión puramente diagnóstica.

## 4. Revisar cada iniciativa ya trabajada

### SC-001 — Comparación de rutas en Windows

1. Revisar el cambio:

   ```powershell
   git diff -- crates/engine/src/io.rs
   ```

2. Ejecutar la regresión:

   ```powershell
   cargo test -p soundcraft-engine stored_media_paths_are_followed_only_on_this_computer -- --exact --nocapture
   ```

3. Si el filtro exacto no encuentra la prueba, repetir sin `--exact`:

   ```powershell
   cargo test -p soundcraft-engine stored_media_paths_are_followed_only_on_this_computer -- --nocapture
   ```

4. Resultado esperado: la prueba aprueba y una ruta equivalente en Windows no se considera un medio externo distinto.

### SC-002 — CI general

1. Abrir `.github/workflows/ci.yml`.
2. Confirmar runners para Linux, macOS y Windows.
3. Confirmar que el workflow ejecuta `cargo xtask ci`.
4. Ejecutar localmente `cargo xtask ci`.
5. Después de subir una rama, abrir GitHub → **Actions** y verificar los tres jobs. Hasta que eso ocurra, el estado correcto sigue siendo “validada localmente, pendiente del runner alojado”.

### SC-003 — Inventario verificable de comandos

1. Regenerar el inventario:

   ```powershell
   cargo xtask capabilities
   ```

2. Comprobar que no quedó obsoleto:

   ```powershell
   cargo run -q -p soundcraft-cli -- capabilities --check docs/command-capabilities.md
   ```

3. Abrir `docs/command-capabilities.md`.
4. Confirmar que cada fila tiene estado y evidencia.
5. Interpretar `not_verified` como “todavía no auditado”, no como “roto”.
6. Revisar la implementación en `crates/engine/src/cmd/mod.rs` y el generador en `apps/soundcraft-cli/src/main.rs`.

### SC-004 — Comandos que simulaban capacidad

Revisar Freeze:

```powershell
cargo test -p soundcraft-engine freeze_does_not_claim_success_without_a_render -- --nocapture
git diff -- crates/engine/src/cmd/track.rs
```

Resultado esperado:

- Crear un freeze nuevo devuelve una limitación explícita.
- Descongelar un estado antiguo continúa permitido.
- La aplicación no afirma que creó audio congelado cuando no lo hizo.

Revisar Renumber Bars:

```powershell
cargo test -p soundcraft-engine renumber_bars_is_explicitly_disabled -- --nocapture
git diff -- crates/engine/src/cmd/event.rs
```

Resultado esperado: el comando aparece como no compatible y no devuelve éxito ficticio.

### SC-005 — Límites de archivos antes de reservar memoria

```powershell
cargo test -p soundcraft-engine file_size_is_rejected_before_reading -- --nocapture
cargo test -p soundcraft-engine hostile_bundles_are_errors_not_panics -- --nocapture
git diff -- crates/engine/src/io.rs
```

Comprobar en el código:

- Límite antes de `read_to_end`.
- Revisión adicional si el archivo crece mientras se lee.
- Límites separados para texto, archivos en memoria, entradas ZIP y tamaño total del bundle.
- Errores recuperables; ningún `panic!`.

### SC-006 — Documentación y métricas coherentes

```powershell
git diff -- ROADMAP.md docs/architecture.md docs/gaps.md docs/hardware-parity.md docs/roadmap.md docs/target-app-parity.md docs/ui-parity.md
```

Para cada cifra o estado modificado, comprobar que apunta a una prueba, comando o limitación concreta. Ejecutar también:

```powershell
cargo xtask parity
cargo xtask capabilities
git diff --check
```

Resultado esperado: no hay contradicciones conocidas entre roadmap, gaps e inventario funcional.

### SC-010 — Navegación y zoom del Piano Roll

Pruebas automatizadas:

```powershell
cargo test -p soundcraft-ui-egui viewport_coordinate_conversion_round_trips -- --nocapture
cargo test -p soundcraft-ui-egui horizontal_zoom_keeps_cursor_tick_stable -- --nocapture
cargo test -p soundcraft-ui-egui viewport_clamps_to_clip_bounds -- --nocapture
```

Prueba manual:

1. Iniciar la aplicación:

   ```powershell
   cargo run --release -p soundcraft -- --demo --control 0
   ```

2. Abrir un clip MIDI en el Piano Roll.
3. Colocar el cursor sobre una nota y hacer zoom horizontal; el tick bajo el cursor debe permanecer estable.
4. Usar zoom vertical; las teclas deben cambiar de alto sin perder alineación con las notas.
5. Desplazarse horizontal y verticalmente.
6. Probar los controles `−`, `Fit` y `+`.
7. Intentar desplazarse antes del inicio y después del final; la vista debe quedar limitada al clip.

### SC-012 — Copiar, cortar, pegar y duplicar notas MIDI

```powershell
cargo test -p soundcraft-engine note_clipboard_commands_are_relative_atomic_and_undoable -- --nocapture
cargo test -p soundcraft-engine note_clipboard_rejects_empty_and_invalid_selections_and_clamps_pitch -- --nocapture
cargo test -p soundcraft-ui-egui piano_roll_clipboard_shortcuts_use_note_commands -- --nocapture
```

Prueba manual:

1. Seleccionar varias notas que no comiencen en el mismo tick.
2. Copiar y pegar en otra posición.
3. Confirmar que conservan intervalos, duraciones y velocidades.
4. Deshacer una vez; todo el pegado debe desaparecer como una sola operación.
5. Rehacer; todo debe reaparecer.
6. Cortar y deshacer.
7. Duplicar y confirmar que no excede los límites del clip ni el rango MIDI 0–127.
8. Confirmar que los atajos actúan sobre notas solo cuando el Piano Roll tiene el contexto activo.

### SC-013 — Etiquetas de todas las notas

```powershell
cargo test -p soundcraft-midi localized_names_keep_every_midi_pitch_and_octave -- --nocapture
cargo test -p soundcraft-ui-egui ui_preferences -- --nocapture
```

Si el segundo filtro no encuentra la prueba, ejecutar todas las pruebas del crate:

```powershell
cargo test -p soundcraft-ui-egui -- --test-threads=1
```

Prueba manual:

1. Abrir el Piano Roll.
2. En el selector de etiquetas probar `Off`, `C / Do only` y `Every key`.
3. Cambiar `English` a `Español`.
4. Cambiar `Sharps (♯)` a `Flats (♭)`.
5. Confirmar ejemplos: MIDI 60 = `C3`/`Do3`; MIDI 61 = `C♯3` o `D♭3`/`Re♭3`, según la preferencia.
6. Cambiar zoom vertical y tamaño de ventana; los nombres deben seguir legibles y alineados.
7. Cerrar y abrir la aplicación; las preferencias deben persistir.
8. Confirmar que cambiar etiquetas no modifica el pitch ni la sesión musical.

### SC-014 — Índice y render visible del Piano Roll

```powershell
cargo test -p soundcraft-ui-egui note_index_returns_only_horizontal_overlaps_and_keeps_long_notes -- --nocapture
cargo test -p soundcraft-ui-egui note_index_matches_linear_search_across_large_varied_sequences -- --nocapture
```

Revisión manual:

1. Abrir un clip con muchas notas.
2. Hacer scroll y zoom rápidamente.
3. Confirmar que las notas largas que empiezan fuera de la vista pero se extienden dentro de ella siguen visibles.
4. Confirmar que selección y hit-test no alcanzan notas invisibles.
5. No declarar completada la optimización hasta capturar tiempos de frame, repaint y uso de CPU con umbrales reproducibles.

### SC-017 — Barra superior y menú adaptativos

```powershell
cargo test -p soundcraft-ui-egui narrow_and_high_dpi_widths_keep_the_essential_toolbar_fixed -- --nocapture
cargo test -p soundcraft-ui-egui narrow_menu_collapses_before_it_can_overlap_the_window_title -- --nocapture
```

Prueba visual offscreen:

```powershell
cargo run -p soundcraft-ui-egui --example ui_shot -- target/review-ui.png
```

Prueba manual:

1. Ejecutar SoundCraft con la sesión demo.
2. Probar una ventana grande, una mediana y una estrecha.
3. Confirmar que transporte y posición permanecen visibles.
4. Confirmar que las acciones secundarias pasan al menú de overflow.
5. Confirmar que el menú principal colapsa antes de superponerse al título.
6. Probar escalado de Windows 100 %, 125 %, 150 % y 200 % si el monitor lo permite.
7. Registrar como pendiente la adaptación de paneles laterales; no confundirla con la barra ya corregida.

## 5. Revisar las iniciativas todavía pendientes

Estas iniciativas se revisan primero como especificación y evidencia; todavía no deben probarse como funciones terminadas.

| ID | Qué revisar ahora | Qué demostraría su cierre |
|---|---|---|
| SC-007 | Callback, locks, asignaciones y trabajo no determinista | Trazas y tests que demuestren callback sin bloqueos/asignaciones en estado estable |
| SC-008 | Dispositivos, canales, duplex y latencia | Grabación/reproducción con hardware real y compensación medida |
| SC-009 | Contrato actual de resampling | Pruebas de calidad, aliasing y selección explícita del algoritmo |
| SC-011 | Ruta de preview y MIDI en vivo | Nota audible con note-on/note-off seguro, sin notas colgadas |
| SC-015 | Escalas, acordes y composición | Operaciones musicales correctas, undo y pruebas por tonalidad |
| SC-016 | MIDI físico | Dispositivos reales, hot-plug, timestamps y pérdida de eventos |
| SC-018 | Semántica accesible | Navegación por teclado, lector de pantalla, foco y contraste |
| SC-019 | Onboarding | Pruebas con principiantes completando los flujos básicos |
| SC-020 | Aislamiento de plugins | Un plugin que cae no termina el proceso principal ni corrompe la sesión |
| SC-021 | Streaming desde disco | Sesiones largas sin cargar todo el audio en RAM ni producir dropouts |
| SC-022 | División de módulos | Capas conservadas, API clara y suite completa sin regresiones |
| SC-023 | Localización | Catálogo traducible, fallback y pruebas de textos largos/RTL si entra en alcance |
| SC-024 | Formatos comprimidos | Exportaciones verificadas por decodificación, metadatos y escucha |
| SC-025 | AAF/OMF/PTX | Alcance legal/técnico definido y fixtures propios interoperables |
| SC-026 | Matriz QA | Casos automatizados, resultados guardados y métricas repetibles |

Para cada ID pendiente:

1. Buscarlo en `docs/audit/13-improvement-backlog.md`.
2. Abrir todos los archivos indicados en **Componentes afectados**.
3. Reproducir la evidencia existente.
4. Marcar cada afirmación como confirmado, probable, potencial o no verificado.
5. Crear primero una prueba que falle cuando se trate de una corrección.
6. Implementar el incremento más pequeño que satisfaga los criterios.
7. Ejecutar la prueba específica.
8. Ejecutar `cargo xtask ci`.
9. Hacer la verificación manual o con hardware indicada.
10. Actualizar estado, evidencia, roadmap y gaps solo después de aprobar todo.

## 6. Revisar visualmente sin iniciar la aplicación principal

El render offscreen evita robar foco:

```powershell
cargo run -p soundcraft-ui-egui --example ui_shot -- target/review-default.png
```

Abrir `target/review-default.png` y comprobar:

- Solapamientos y texto cortado.
- Contraste y estados seleccionados.
- Controles esenciales visibles.
- Alineación del teclado y notas MIDI.
- Estados vacíos y mensajes de error.

La imagen confirma composición visual, pero no sustituye la prueba de interacción, teclado, audio o persistencia.

## 7. Plantilla para registrar el resultado de cada revisión

Copiar esta tabla debajo de la iniciativa correspondiente:

```markdown
| Campo | Resultado |
|---|---|
| Fecha y equipo |  |
| Commit probado |  |
| Plataforma |  |
| Comando o pasos |  |
| Resultado esperado |  |
| Resultado observado |  |
| Evidencia adjunta |  |
| Estado | CONFIRMADO / PROBABLE / POTENCIAL / NO VERIFICADO |
| Regresión encontrada | Sí / No |
| Acción siguiente |  |
```

## 8. Criterio final para aprobar una iniciativa

Una iniciativa solo puede marcarse **completada** cuando:

1. Sus criterios de aceptación están satisfechos.
2. Las pruebas específicas aprueban.
3. `cargo xtask ci` aprueba.
4. La revisión manual requerida aprueba.
5. No hay regresiones de undo, persistencia, tiempo real o accesibilidad.
6. La documentación coincide con el comportamiento real.
7. Las limitaciones de hardware o entorno están documentadas.

Si falta cualquiera de estos puntos, usar **parcial**, **probable**, **potencial** o **no verificado**, según corresponda.
