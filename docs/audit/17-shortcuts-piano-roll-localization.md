# SoundCraft — Piano Roll, atajos e idiomas

**Fecha:** 2026-10-18
**Plataforma auditada:** Windows
**Convención:** los manifiestos internos guardan `Cmd`; SoundCraft lo presenta y procesa como `Ctrl` en Windows/Linux.

## Resultado ejecutivo

- El Piano Roll puede cambiarse a tamaños predefinidos o ajustarse con un slider; el separador superior continúa permitiendo ajuste libre. El tamaño queda guardado en el workspace.
- El Note Resizing toma el borde derecho desde el punto exacto de presión, ajusta el extremo de la nota a la rejilla, aplica el mismo cambio de duración a toda la selección y permite omitir el snap con `Alt`; la selección permanece activa y todo el grupo se deshace en un paso.
- Un clic derecho elimina una nota; mantenerlo y arrastrar borra todas las notas cruzadas. El recorrido completo se agrupa en un solo Deshacer.
- Los clips se pueden copiar/cortar/duplicar desde su menú contextual y pegar con **Paste Here** en la pista y posición exactas señaladas.
- `Ctrl+C` seguido de `Ctrl+V` sobre un clip seleccionado ya no lo superpone de forma invisible: busca el primer intervalo libre después de la selección. En el Piano Roll, el grupo de notas se pega en el siguiente compás libre, conserva su geometría y amplía el clip cuando es necesario.
- La mayoría de los atajos solicitados ya existían. Se añadieron Dashboard, Commands Keyboard Focus, Smart Tool, ciclo de herramientas, preset de pista y bypass por categoría.
- Se incorporó una preferencia persistente de idioma para inglés, español, portugués, francés, alemán, ruso y japonés. La primera migración cubre menús principales, Dashboard, Apariencia y Piano Roll; las cadenas no migradas usan inglés, nunca un identificador crudo.
- No se asignaron atajos falsos a funciones inexistentes o indistinguibles con la capa de entrada actual.

## Piano Roll

| Requisito | Estado | Evidencia | Verificación |
|---|---|---|---|
| Ampliar/reducir panel | Implementado | `midi_editor.rs::MidiEditorState`, menú `Size`; `docking.rs::resize_midi_panel` | Prueba `midi_size_request_resizes_the_dock_and_persists` |
| Ajuste libre | Ya existía, ahora descubrible | divisor del dock + texto de ayuda en `Size` | pruebas de arrastre existentes en `midi_panel.rs` |
| Presets de tamaño | Implementado | Compact 260, Comfortable 420, Large 600, Maximum | prueba de resize y persistencia |
| Note Resizing estilo FL Studio | Implementado | borde derecho capturado desde `press_origin` + `midi.notes_resize`; el extremo se ajusta al grid, la selección comparte delta y `Alt` omite snap | prueba UI del gesto y prueba engine de grupo/undo atómico |
| Clic derecho elimina nota | Implementado | `midi_editor.rs`, `PointerButton::Secondary` → comando de borrado combinado | clic elimina una; arrastre elimina las cruzadas; un Deshacer restaura el gesto |
| Copiar/pegar clips con ratón | Implementado | `edit_window.rs`, **Copy Clip** y **Paste Here** | prueba de posición exacta y conservación del original |
| Copiar/pegar clips con teclado | Implementado | `shortcuts.rs::free_clip_paste_target`; `edit.paste` limpia la selección obsoleta | eventos egui con `Ctrl` real de Windows y pegado inmediato sin mover el cursor |
| Copiar/pegar notas al siguiente compás libre | Implementado | `midi_editor.rs::next_free_bar_for_notes`; `midi.notes_paste` amplía el clip | prueba de grupo, compás ocupado, pegados sucesivos y extensión/undo |
| No afectar edición ni audio | Implementado | altura vive en estado UI/workspace; borrar usa comando engine | suite UI y engine |

## 1. Sesiones

| Acción solicitada | Atajo | Estado |
|---|---|---|
| Nueva sesión | `Ctrl+N` | Ya existía: `session.new` |
| Abrir sesión | `Ctrl+O` | Ya existía: `session.open` |
| Guardar | `Ctrl+S` | Ya existía: `session.save` |
| Guardar como | solicitado `Ctrl+Win+S` | Se conserva `Ctrl+Shift+S`. egui no expone la tecla Windows como modificador portable y el sistema puede reservarla; no se sustituyó un atajo funcional por uno no detectable |
| Cerrar sesión | `Ctrl+Shift+W` | Ya existía: `session.close` |
| Dashboard | `Ctrl+Alt+O` | Añadido: `window.dashboard`; ofrece Nueva, Abrir y sesiones recientes |
| Importar audio | `Ctrl+Shift+I` | Ya existía |
| Importar datos de sesión | `Alt+Shift+I` | Ya existía |
| Bounce Mix | `Ctrl+Alt+B` | Ya existía |
| Nueva pista | `Ctrl+Shift+N` | Ya existía |

## 2. Transporte y grabación

| Acción solicitada | Atajo | Estado |
|---|---|---|
| Reproducir/detener | `Espacio` | Ya existía |
| Media velocidad | `Shift+Espacio` | Ya existía para reproducción |
| Preparar/iniciar grabación | `Ctrl+Espacio` | Ya existía; la lógica exige pista armada y muestra indicación si no la hay |
| Iniciar grabación | `F12` | Ya existía |
| Loop playback | `Ctrl+Shift+L` | Ya existía |
| Monitoreo de entrada de pistas armadas | `Alt+K` | Ya existía |
| Grabar a media velocidad | `Ctrl+Shift+Espacio` | No implementado: solo existe `TransportRequest::HalfSpeed` para reproducción. Requiere contrato de captura, compensación temporal y pruebas de dispositivo; asignar la tecla sin eso sería falso |

## 3. Edición

Todos estos atajos ya estaban registrados y ejecutan comandos reales: `Ctrl+Z`, `Ctrl+Shift+Z`, `Ctrl+X`, `Ctrl+C`, `Ctrl+V`, `Ctrl+A`, `Ctrl+B`, `Ctrl+E`, `Ctrl+D`, `Alt+R`, `Ctrl+T`, `Alt+Shift+3`, `Ctrl+M`, `Ctrl+H`, `Ctrl+Shift+E` y `Ctrl+U`.

Corresponden, respectivamente, a Deshacer, Rehacer, Cortar, Copiar, Pegar, Seleccionar todo, Clear sin ripple, Separar, Duplicar, Repetir, Trim a selección, Consolidar, Mute clip, Heal Separation, Insertar silencio y Strip Silence.

## 4. Herramientas y modos

| Función | Estado |
|---|---|
| `F5` Zoom, `F6` Trim, `F7` Selector, `F8` Grabber, `F9` Scrubber, `F10` Pencil | Ya existían |
| Smart Tool con `F6+F7` o `F7+F8`, en cualquier orden | Añadido |
| `Esc` alterna herramientas | Añadido; recorre Zoom, Trim, Selector, Grabber, Scrubber, Pencil y Smart |
| `F1` Shuffle, `F2` Slip, `F3` Spot, `F4` Grid | Ya existían |
| `Shift+F4` Snap to Grid | Ya existía |

## 5. Commands Keyboard Focus

Se añadió `Ctrl+Alt+1` para alternar el modo y se verificó que tiene prioridad sobre la familia de rating de clips. Ya estaban implementadas las letras:

| Teclas | Estado |
|---|---|
| `A`, `S` | Trim inicio/final |
| `D`, `F`, `G` | Fade in, crossfade/fades, fade out |
| `B`, `C`, `V`, `X`, `Z` | Separar, copiar, pegar, cortar, deshacer |
| `R`, `T`, `E`, `N` | Zoom out/in, zoom toggle, follow playback |
| `H` alinear inicio | Pendiente: no existe aún un comando engine inequívoco para esta semántica; debe diseñarse con selección múltiple, sync point, Grid/Slip y undo antes de enlazar la tecla |

## 6. Teclado numérico

Estado: **pendiente de una capa de entrada con ubicación física de tecla**. La versión actual de egui representa `Num0…Num9` como tecla de la fila principal **o** del keypad y no permite distinguirlas. Secuestrar `0…9` afectaría escritura, ratings, presets de zoom y Commands Focus.

Plan seguro:

1. Capturar `PhysicalKey::Numpad0…Numpad9` en la integración winit.
2. Exponer eventos numpad separados a `shortcuts.rs`.
3. Añadir preferencia `numeric_keypad_mode = transport|classic|shuttle`.
4. Implementar comandos faltantes: loop record, QuickPunch, countoff y MIDI Merge/Replace.
5. Probar teclado completo, portátil sin numpad, Num Lock y layouts internacionales.

No se marcó como implementado ninguno de estos diez accesos porque hoy no se puede garantizar que provengan del keypad.

## 7. Pistas y mezcla

| Acción | Estado |
|---|---|
| Nueva, duplicar, agrupar, crear carpeta, commit y bounce de pista | Ya existían con los atajos solicitados |
| Guardar preset de pista `Alt+Shift+P` | Atajo añadido al comando real existente |
| Bypass EQ `Shift+E` | Atajo añadido |
| Bypass Dynamics `Shift+C` | Atajo añadido |
| Bypass Reverb `Shift+V` | Atajo añadido |
| Bypass Delay `Shift+D` | Atajo añadido |

Los bypass actúan sobre las pistas seleccionadas y la categoría registrada del plugin. Si un plugin no declara la categoría esperada, no se clasifica artificialmente.

## 8. Mouse y modificadores

| Gesto solicitado | Estado observado |
|---|---|
| `Shift+clic` pistas consecutivas | Implementado en `edit_window::click_track` |
| `Ctrl+clic` pistas independientes | Implementado; egui lo normaliza como `command` fuera de macOS |
| `Ctrl+arrastrar` ajuste fino | Implementado en clip gain; otros faders no son todavía uniformes (por ejemplo send usa Shift) |
| `Alt+clic` aplicar/restablecer globalmente | Parcial y dependiente del control; no existe contrato uniforme para toda la aplicación |
| `Alt+Shift+clic` aplicar a selección | No verificado como regla global |
| Doble clic con Selector sobre clip | Implementado: selecciona el clip completo |
| Triple clic para pista completa | No implementado/verificado |

Plan: definir una política única de modificadores en `widgets`, migrar control por control y añadir pruebas de interacción antes de anunciar compatibilidad global.

## 9. Idiomas

La preferencia se encuentra en **Window → Languages & Themes → Interface language** y se guarda con `UiState`. El mismo panel ofrece Studio Dark/Light, Nord Studio, Dracula Studio, Midnight Cyber, seguimiento del sistema e importación de temas JSON validados. Su identidad interna es estable, por lo que no se mueve al traducirse el título.

| Idioma | Código interno | Cobertura actual |
|---|---|---|
| English | `english` | Fuente completa |
| Español | `spanish` | Núcleo migrado |
| Português | `portuguese` | Núcleo migrado |
| Français | `french` | Núcleo migrado |
| Deutsch | `german` | Núcleo migrado |
| Русский | `russian` | Núcleo migrado |
| 日本語 | `japanese` | Núcleo migrado + fallback de fuente del sistema en escritorio |

`crates/ui-egui/src/i18n.rs` contiene el catálogo inicial tipado. La notación musical del Piano Roll sigue siendo una preferencia separada: cambiar el idioma de la interfaz no cambia MIDI, pitch, octava ni la elección C/Do.

### Trabajo restante para localización completa

1. Migrar las 512 etiquetas del catálogo de comandos sin cambiar sus IDs.
2. Migrar diálogos, tooltips, errores y textos dinámicos/plurales.
3. Extraer catálogos externos versionados y añadir pseudo-localización.
4. Revisar cada idioma con hablante nativo.
5. Añadir fallback CJK al build web; el fallback actual usa fuentes instaladas por el sistema de escritorio.
6. Probar expansión de texto a 200 %, 960×600, DPI alto e IME japonés.

## Evidencia automatizada

- `midi_panel`: tamaño del panel, persistencia, clic derecho simple/arrastrado y undo agrupado.
- `midi_panel`: `Ctrl+C`/`Ctrl+V` real de Windows coloca la copia en el siguiente compás libre y conserva el patrón.
- `clip_menu`: copiar y pegar un clip en la pista y posición del clic derecho.
- `cmd::midi`: redimensionamiento múltiple con duración mínima, extensión del clip al pegar y undo atómico.
- `shortcuts`: pegado inmediato de clips al primer intervalo libre, Commands Focus, ciclo por Escape y metadatos de los nuevos atajos.
- `i18n`: cada idioma traduce el menú raíz, fallback inglés y round-trip serde.
- `UiState`: preferencias antiguas conservan valores por defecto y el idioma nuevo persiste.

## Limitaciones explícitas

- No se probó hardware MIDI ni interfaz de audio en esta entrega.
- No se afirma localización completa: es una base funcional y una primera superficie migrada.
- No se implementaron media velocidad de grabación, keypad Transport ni `H` hasta contar con comandos/entrada correctos.
- Los atajos dependientes del sistema operativo pueden estar reservados por Windows o por utilidades de terceros.
