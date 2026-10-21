# SoundCraft — Manual paso a paso para el usuario final

**Fecha de revisión:** 2026-10-18
**Plataforma de referencia:** Windows
**Alcance:** sesiones, transporte, grabación, edición, herramientas, pistas, mezcla, Piano Roll, atajos e idioma.

Este documento explica cómo usar las funciones desde la interfaz. No requiere conocer Rust, comandos internos ni la arquitectura del proyecto.

## 1. Antes de comenzar

### 1.1 Abrir SoundCraft

Si SoundCraft está instalado:

1. Abrir **SoundCraft** desde el menú Inicio o desde su acceso directo.
2. Esperar a que aparezcan la ventana Edit, la lista de pistas y el transporte.
3. Si aparece el Dashboard, elegir una sesión reciente, **New Session…** u **Open Session…**.

Si se está probando desde el repositorio:

```powershell
cargo run --release -p soundcraft -- --demo
```

### 1.2 Convenciones de este manual

- En Windows y Linux, los atajos indicados usan `Ctrl`.
- En macOS, normalmente debe sustituirse `Ctrl` por `Cmd`.
- Una pista se selecciona haciendo clic en su nombre o cabecera, a la izquierda de la línea de tiempo.
- Un clip se selecciona haciendo clic sobre el bloque de audio o MIDI de la línea de tiempo.
- Una selección temporal se crea con la herramienta Selector (`F7`) y arrastrando sobre una pista.
- Los atajos no actúan dentro de un campo de texto. Hacer clic antes en la ventana Edit, Mix o Piano Roll según corresponda.
- Algunos textos secundarios todavía aparecen en inglés aunque se elija otro idioma.

### 1.3 Estados usados

- **Disponible:** la acción tiene interfaz y lógica funcional.
- **Disponible con condición:** funciona cuando existe la selección, pista, clip, dispositivo o plugin requerido.
- **Parcial:** existe, pero no se comporta de forma uniforme en toda la interfaz.
- **Pendiente:** no hay una operación segura que el usuario pueda ejecutar todavía.

## 2. Crear, abrir, guardar y administrar sesiones

### 2.1 Crear una sesión nueva — `Ctrl+N`

1. Pulsar `Ctrl+N` o abrir **File → New…**.
2. Escribir el nombre de la sesión.
3. Elegir el sample rate, por ejemplo `48000 Hz`.
4. Activar **Start from the demo session** solo si se desea partir del proyecto de demostración.
5. Pulsar **Create**.
6. Guardar inmediatamente con `Ctrl+S` para definir la ubicación del proyecto.

Si hay cambios sin guardar en la sesión anterior, SoundCraft pregunta si se desean guardar antes de sustituirla.

### 2.2 Abrir una sesión — `Ctrl+O`

1. Pulsar `Ctrl+O` o abrir **File → Open Session…**.
2. Pulsar **Browse…**.
3. Seleccionar el archivo `.scraft` de la sesión.
4. Confirmar con **OK/Open**.
5. Si SoundCraft informa archivos externos ausentes, no continuar una edición destructiva hasta volver a vincular o recuperar esos archivos.

También se puede abrir el Dashboard con `Ctrl+Alt+O` y elegir una sesión reciente.

### 2.3 Guardar la sesión — `Ctrl+S`

1. Pulsar `Ctrl+S`.
2. Si es el primer guardado, elegir una carpeta y un nombre.
3. Confirmar con **OK**.
4. Si el destino ya existe, revisar la ruta y confirmar **Replace** únicamente si realmente se desea reemplazarlo.

Los siguientes `Ctrl+S` guardan en el mismo destino.

### 2.4 Guardar como — `Ctrl+Shift+S`

1. Pulsar `Ctrl+Shift+S` o abrir **File → Save Session As…**.
2. Elegir otra carpeta o escribir otro nombre.
3. Confirmar con **OK**.
4. Comprobar que el nuevo nombre aparece como la sesión activa.

`Ctrl+Win+S` no se usa: la tecla Windows está reservada por el sistema y no se expone de forma portable. El atajo funcional es `Ctrl+Shift+S`.

### 2.5 Cerrar la sesión — `Ctrl+Shift+W`

1. Pulsar `Ctrl+Shift+W` o abrir **File → Close Session**.
2. Si hay cambios pendientes, elegir guardar, descartar o cancelar.
3. Confirmar que se cierra la sesión actual sin cerrar necesariamente la aplicación.

### 2.6 Abrir o cerrar el Dashboard — `Ctrl+Alt+O`

1. Pulsar `Ctrl+Alt+O`.
2. Desde el Dashboard, elegir:
   - **New Session…** para crear un proyecto.
   - **Open Session…** para buscar un proyecto.
   - Una entrada de sesiones recientes para reabrirla.
3. Pulsar otra vez `Ctrl+Alt+O` o cerrar la ventana para ocultarlo.

### 2.7 Importar audio — `Ctrl+Shift+I`

1. Colocar el cursor de edición en el punto donde debe comenzar el audio.
2. Pulsar `Ctrl+Shift+I` o abrir **File → Import → Audio…**.
3. Pulsar **Browse…** y elegir un archivo compatible, como WAV, AIFF, FLAC o MP3.
4. Confirmar la importación.
5. SoundCraft crea una pista nueva por defecto y coloca el clip en la posición de edición.
6. Reproducir el inicio y verificar canal, duración y sample rate percibido antes de seguir trabajando.

### 2.8 Importar datos de otra sesión — `Alt+Shift+I`

1. Guardar la sesión actual.
2. Pulsar `Alt+Shift+I` o abrir **File → Import → Session Data…**.
3. Elegir el archivo `.scraft` de origen.
4. Confirmar la importación.
5. Revisar las pistas incorporadas, sus clips, mezclador, automatización y referencias de audio.
6. Guardar la sesión de destino con un nombre nuevo si se desea conservar intacta la versión anterior.

Actualmente la interfaz importa los datos de la sesión indicada; todavía no ofrece un selector visual detallado de pistas antes de ejecutar la importación.

### 2.9 Exportar la mezcla — `Ctrl+Alt+B`

1. Para exportar solo una parte, crear antes una selección temporal con `F7` y arrastrar sobre la línea de tiempo.
2. Para exportar toda la canción, dejar la selección temporal vacía.
3. Pulsar `Ctrl+Alt+B` o abrir **File → Bounce Mix…**.
4. En **File**, escribir la ruta o pulsar **Browse…**.
5. Elegir el formato: WAV, AIFF o FLAC.
6. Elegir la profundidad: 16 bits, 24 bits o 32 bits flotantes, según el formato.
7. Activar **Normalize to -0.1 dBFS** solo si se desea normalización automática.
8. Pulsar **Bounce**.
9. Esperar a que finalice la ventana de progreso.
10. Reproducir el archivo exportado fuera de SoundCraft y comprobar principio, final, canales y nivel.

### 2.10 Crear una pista — `Ctrl+Shift+N`

1. Pulsar `Ctrl+Shift+N` o abrir **Track → New…**.
2. Indicar cuántas pistas crear, de 1 a 128.
3. Elegir el formato de canales, por ejemplo Mono o Stereo.
4. Elegir el tipo: Audio, MIDI, Instrument, Aux, Master, VCA o Folder.
5. Elegir la base de tiempo:
   - **Samples** para audio alineado al tiempo absoluto.
   - **Ticks** para material musical que debe seguir tempo y compás.
6. Escribir un nombre.
7. Pulsar **Create**.

Para tocar un instrumento desde MIDI, crear una pista **Instrument**. Una pista **MIDI** almacena notas, pero necesita ser enrutada a un instrumento para producir sonido.

## 3. Reproducción, bucle y grabación

### 3.1 Reproducir y detener — `Espacio`

1. Hacer clic en la regla o línea de tiempo para colocar el cursor de reproducción.
2. Pulsar `Espacio` para reproducir.
3. Pulsar `Espacio` de nuevo para detener.

Si no se oye audio, revisar el dispositivo en **Setup → Playback Engine…**, la salida de la pista, Mute/Solo y la salida Master.

### 3.2 Reproducir a media velocidad — `Shift+Espacio`

1. Colocar el cursor donde debe comenzar la reproducción.
2. Mantener `Shift` y pulsar `Espacio`.
3. Detener con `Espacio`.

Esta función modifica la reproducción. No implica grabación a media velocidad.

### 3.3 Activar reproducción en bucle — `Ctrl+Shift+L`

1. Con `F7`, arrastrar sobre la región temporal que se quiere repetir.
2. Pulsar `Ctrl+Shift+L`.
3. Confirmar que el control de loop queda activo.
4. Pulsar `Espacio` para reproducir el rango repetidamente.
5. Pulsar `Ctrl+Shift+L` otra vez para desactivar el bucle.

### 3.4 Preparar una pista para grabar

1. Crear o seleccionar una pista Audio o Instrument.
2. En una pista Audio, seleccionar una entrada válida en su cabecera o canal de mezcla.
3. Pulsar el botón rojo de armado de la pista, **Record enable**.
4. Si se necesita escuchar la entrada, pulsar el botón `I` de **Input monitoring**.
5. Ajustar el nivel en la interfaz de audio para evitar clipping.
6. Colocar el cursor o crear el rango de grabación.

### 3.5 Iniciar y detener la grabación — `Ctrl+Espacio` o `F12`

1. Completar primero la preparación anterior.
2. Pulsar `Ctrl+Espacio` o `F12`.
3. Interpretar la toma mientras el transporte está grabando.
4. Pulsar `Espacio` para detener.
5. Revisar el clip creado.
6. Pulsar `Ctrl+S` para guardar la toma.

Si ninguna pista está armada, SoundCraft no debe iniciar una captura válida y muestra una indicación de preparación.

### 3.6 Monitoreo de entrada

Hay dos acciones relacionadas:

1. Para alternar el monitoreo de las pistas seleccionadas, seleccionarlas y pulsar `Shift+I`.
2. Para poner en modo Input Only todas las pistas que ya estén armadas, pulsar `Alt+K`.

`Alt+K` no arma pistas por sí mismo; primero se debe activar **Record enable** en cada pista necesaria.

### 3.7 Grabación a media velocidad — pendiente

`Ctrl+Shift+Espacio` todavía no inicia una grabación a media velocidad. No debe utilizarse para una toma real hasta que exista compensación temporal, captura y pruebas con dispositivos.

## 4. Preparar una edición de clips

Antes de usar la mayoría de los atajos de esta sección:

1. Abrir la ventana Edit.
2. Pulsar `F7` para elegir Selector.
3. Seleccionar una o más pistas desde sus cabeceras.
4. Arrastrar sobre la línea de tiempo para crear un rango, o hacer clic en un clip cuando la acción opere sobre clips.
5. Confirmar visualmente qué pistas, clips y tiempo están seleccionados.

## 5. Editar audio y clips

### 5.1 Deshacer y rehacer

1. Ejecutar una edición.
2. Pulsar `Ctrl+Z` para deshacerla.
3. Pulsar `Ctrl+Shift+Z` para rehacerla.
4. Revisar el texto del menú Edit si se necesita saber qué acción se deshará o rehará.

### 5.2 Cortar — `Ctrl+X`

1. Seleccionar el rango que se quiere retirar.
2. Pulsar `Ctrl+X`.
3. Colocar el cursor en el destino.
4. Pulsar `Ctrl+V` para pegarlo.

### 5.3 Copiar — `Ctrl+C`

1. Seleccionar un clip o rango en la ventana Edit.
2. Pulsar `Ctrl+C`.
3. Para una copia rápida, pulsar `Ctrl+V` sin cambiar la selección: SoundCraft la coloca en el primer intervalo libre después del material seleccionado.
4. Para elegir un destino exacto, hacer clic allí para colocar el cursor de edición —esto retira la selección del clip— y pulsar `Ctrl+V`.

### 5.4 Pegar — `Ctrl+V`

1. Cortar o copiar contenido.
2. Seleccionar la pista de destino.
3. Colocar el cursor en el tiempo exacto.
4. Pulsar `Ctrl+V`.
5. Verificar que el contenido quedó en la pista y tiempo esperados.

Si se mantiene seleccionado el clip que se acaba de copiar y se pulsa inmediatamente `Ctrl+V`, no se pega encima del original: SoundCraft comienza al final de la selección y busca el primer intervalo libre en las pistas implicadas.

### 5.4.1 Copiar y pegar un clip con el ratón

1. Elegir Grabber con `F8`.
2. Hacer clic derecho sobre el clip de origen.
3. Elegir **Copy Clip**.
4. Hacer clic derecho en la pista y posición temporal exactas donde debe empezar la copia.
5. Elegir **Paste Here**.
6. Confirmar que el original continúa intacto y que la copia conserva contenido, duración y propiedades.
7. Pulsar `Ctrl+Z` una vez para retirar solo el pegado si el destino no era correcto.

También se puede hacer clic en el clip y pulsar `Ctrl+C`, `Ctrl+V` para crear una copia rápida en el primer intervalo libre. Para pegar en un punto exacto, colocar allí el cursor de edición antes de `Ctrl+V`. **Paste Here** permanece deshabilitado mientras no exista contenido compatible en el portapapeles de clips.

### 5.5 Seleccionar todo — `Ctrl+A`

1. Hacer clic en la ventana Edit, fuera de campos de texto.
2. Pulsar `Ctrl+A`.
3. Confirmar la selección antes de borrar, mover o procesar.

En el Piano Roll, `Ctrl+A` se aplica al contexto MIDI cuando ese editor tiene el foco.

### 5.6 Eliminar sin desplazar clips — `Ctrl+B`

1. Crear una selección temporal sobre las pistas afectadas.
2. Pulsar `Ctrl+B`.
3. Confirmar que el contenido desaparece y el material posterior conserva su posición.
4. Usar `Ctrl+Z` si se seleccionó un rango incorrecto.

### 5.7 Dividir en el cursor o en la selección — `Ctrl+E`

1. Colocar el cursor dentro del clip, o crear un rango.
2. Pulsar `Ctrl+E`.
3. Confirmar los nuevos límites de clip.

### 5.8 Duplicar selección — `Ctrl+D`

1. Seleccionar el material.
2. Pulsar `Ctrl+D`.
3. Confirmar que la copia aparece inmediatamente después de la selección original.

### 5.9 Repetir selección — `Alt+R`

1. Seleccionar el patrón que se quiere repetir.
2. Pulsar `Alt+R`.
3. Indicar el número de repeticiones.
4. Confirmar con **OK**.

### 5.10 Recortar el clip a la selección — `Ctrl+T`

1. Seleccionar el tramo que se desea conservar dentro de uno o más clips.
2. Pulsar `Ctrl+T`.
3. Confirmar que las partes exteriores dejan de formar parte del clip visible.

### 5.11 Consolidar — `Alt+Shift+3`

1. Seleccionar el rango completo que debe convertirse en un clip continuo.
2. Pulsar `Alt+Shift+3`.
3. Esperar el procesamiento.
4. Confirmar que aparece un clip consolidado que cubre el rango.
5. Escucharlo antes de borrar o archivar el material de origen.

### 5.12 Silenciar o activar un clip — `Ctrl+M`

1. Seleccionar uno o más clips.
2. Pulsar `Ctrl+M`.
3. Confirmar el cambio visual y reproducir para verificar que el estado cambió.
4. Pulsar `Ctrl+M` otra vez para alternarlo.

### 5.13 Reparar una división — `Ctrl+H`

1. Seleccionar la separación entre dos clips que proceden del mismo material compatible.
2. Pulsar `Ctrl+H`.
3. Confirmar que la separación se elimina.

No se pueden unir de esta forma clips de fuentes incompatibles.

### 5.14 Insertar silencio — `Ctrl+Shift+E`

1. Seleccionar las pistas y el rango donde se necesita espacio.
2. Pulsar `Ctrl+Shift+E`.
3. Confirmar que se inserta silencio en el rango seleccionado.
4. Revisar sincronía, automatización y marcadores relacionados.

### 5.15 Eliminar silencios automáticamente — `Ctrl+U`

1. Seleccionar uno o más clips de audio.
2. Pulsar `Ctrl+U`.
3. En **Strip Silence**, ajustar:
   - Umbral de detección.
   - Duración mínima.
   - Margen previo.
   - Margen posterior.
4. Confirmar la operación.
5. Escuchar todas las transiciones; si se cortan ataques o colas, deshacer y usar un umbral menos agresivo o más margen.

## 6. Herramientas de edición

### 6.1 Zoomer — `F5`

1. Pulsar `F5`.
2. Arrastrar sobre la zona que se desea ampliar.
3. Usar los controles de zoom o seleccionar otra herramienta para salir.

### 6.2 Trimmer — `F6`

1. Pulsar `F6`.
2. Colocar el puntero en el borde inicial o final de un clip.
3. Arrastrar el borde hasta la posición deseada.

### 6.3 Selector — `F7`

1. Pulsar `F7`.
2. Hacer clic para colocar el cursor.
3. Arrastrar para crear una selección temporal.

### 6.4 Grabber — `F8`

1. Pulsar `F8`.
2. Arrastrar un clip para moverlo.
3. Hacer doble clic sobre un clip MIDI para abrirlo en el Piano Roll.

### 6.5 Scrubber — `F9`

1. Pulsar `F9`.
2. Arrastrar sobre el material para inspeccionarlo mientras se desplaza.
3. Soltar al encontrar el punto buscado.

### 6.6 Pencil — `F10`

1. Pulsar `F10`.
2. Dibujar en una superficie compatible, como notas MIDI o automatización.
3. Cambiar a Selector o Grabber para editar lo creado.

### 6.7 Smart Tool — `F6+F7` o `F7+F8`

1. Mantener presionada una tecla del par.
2. Pulsar la segunda tecla.
3. Confirmar que Smart Tool queda seleccionada.
4. Mover el cursor por distintas zonas del clip; la operación disponible depende de la posición del puntero.

### 6.8 Recorrer herramientas — `Esc`

1. Hacer clic en la ventana Edit.
2. Pulsar `Esc` repetidamente.
3. Observar cómo cambia la herramienta activa.

## 7. Modos de edición y ajuste a rejilla

### 7.1 Shuffle — `F1`

1. Pulsar `F1`.
2. Mover o retirar un clip.
3. Verificar cómo los clips vecinos se acomodan según el modo Shuffle.

### 7.2 Slip — `F2`

1. Pulsar `F2`.
2. Mover un clip libremente.
3. Usar el valor de Nudge cuando se necesiten desplazamientos repetibles.

### 7.3 Spot — `F3`

1. Pulsar `F3`.
2. Mover o colocar un clip.
3. Introducir o confirmar la posición exacta en el diálogo Spot cuando aparezca.

### 7.4 Grid — `F4`

1. Elegir el valor de Grid en la barra superior.
2. Pulsar `F4`.
3. Mover, crear o recortar material.
4. Confirmar que los límites se ajustan a la rejilla.

### 7.5 Snap to Grid — `Shift+F4`

1. Pulsar `Shift+F4` para alternar el ajuste a rejilla.
2. Mover un clip cerca de una línea de grid.
3. Confirmar si se alinea o se mueve libremente según el estado elegido.

## 8. Commands Keyboard Focus

Este modo permite editar con una sola letra. Debe desactivarse al escribir nombres o texto.

### 8.1 Activarlo o desactivarlo

1. Abrir la ventana Edit.
2. Pulsar `Ctrl+Alt+1` o hacer clic en el botón `a–z` de la barra.
3. Confirmar que el botón cambia de estado.
4. Repetir el atajo para desactivarlo.

### 8.2 Usar sus teclas

Con el modo activo y una selección válida:

- `A`: recorta el inicio hasta el cursor o selección.
- `S`: recorta el final.
- `D`: crea un fade desde el inicio.
- `F`: abre o crea fades/crossfade según la selección.
- `G`: crea un fade hacia el final.
- `B`: divide el material.
- `C`: copia.
- `V`: pega.
- `X`: corta.
- `Z`: deshace.
- `R`: aleja el zoom.
- `T`: acerca el zoom.
- `E`: alterna el zoom de edición.
- `N`: alterna que la inserción siga la reproducción.

La tecla `H` para alinear inicios está **pendiente**. No se ha vinculado porque todavía no existe una regla inequívoca para sync points, selección múltiple, Grid/Slip y deshacer.

## 9. Piano Roll y edición MIDI

### 9.1 Abrir un clip MIDI

1. Crear o seleccionar una pista MIDI o Instrument.
2. Localizar un clip MIDI en la línea de tiempo.
3. Elegir Grabber con `F8`.
4. Hacer doble clic sobre el clip.
5. Si el panel no aparece, abrir **Window → MIDI Editor**.

### 9.2 Aumentar el tamaño del Piano Roll

1. Abrir el Piano Roll.
2. Pulsar **Size** en la esquina superior derecha.
3. Elegir uno de los presets:
   - **Compact**: vista pequeña.
   - **Comfortable**: edición normal.
   - **Large**: mayor precisión vertical.
   - **Maximum**: prioriza el Piano Roll.
4. Para un tamaño exacto, mover el control **Piano Roll height**.
5. Como alternativa, arrastrar verticalmente el separador situado encima del Piano Roll.
6. Cerrar y volver a abrir SoundCraft para comprobar que el tamaño se conserva.

Para acertar con más facilidad en una nota, combinar un panel **Large** con zoom vertical mediante `Alt + rueda`.

### 9.3 Crear una nota

1. Elegir Pencil con `F10`.
2. Elegir el valor de rejilla.
3. Hacer clic en la fila del pitch y columna temporal deseadas.
4. Reproducir el clip para verificar la nota.

### 9.4 Seleccionar notas

1. Elegir Selector.
2. Hacer clic en una nota para seleccionarla.
3. Mantener `Shift` y hacer clic en otras para añadirlas.
4. Arrastrar desde un espacio vacío para encerrar varias notas.
5. Pulsar `Ctrl+A` con el Piano Roll activo para seleccionar todas las notas del contexto.

### 9.5 Mover y transponer notas

1. Seleccionar una o varias notas.
2. Arrastrar horizontalmente para cambiar su tiempo.
3. Arrastrar verticalmente para cambiar pitch.
4. Usar flecha arriba o abajo para mover un semitono.
5. Usar `Shift + arriba/abajo` para mover una octava.
6. Usar izquierda o derecha para desplazar por pasos de grid.

### 9.6 Cambiar la duración

1. Colocar el puntero en el borde derecho de una nota.
2. Esperar a que aparezca el cursor horizontal de doble flecha.
3. Arrastrar el borde hacia la izquierda para acortar o hacia la derecha para alargar.
4. Soltar en la duración deseada; por defecto el extremo queda ajustado a la rejilla activa.
5. Mantener `Alt` durante el arrastre para omitir temporalmente el snap y hacer un ajuste libre.

Para redimensionar varias notas como en el Note Resizing de FL Studio:

1. Seleccionar las notas con `Shift+clic`, selección rectangular o `Ctrl+A`.
2. Tomar el borde derecho de cualquiera de las notas seleccionadas.
3. Arrastrar: todas cambian su duración con el mismo desplazamiento temporal, conservando sus inicios, pitches y velocidades.
4. Si una nota llegaría a duración cero, SoundCraft la limita a un tick.
5. La selección permanece activa para poder repetir el ajuste, mover o copiar el grupo.
6. Pulsar `Ctrl+Z` una vez para restaurar todo el redimensionamiento del grupo.

### 9.7 Eliminar con clic derecho

1. Colocar el puntero exactamente sobre una nota.
2. Hacer clic con el botón derecho.
3. La nota apuntada se elimina inmediatamente.
4. Pulsar `Ctrl+Z` si fue accidental.

Para borrar varias notas de un solo gesto:

1. Mantener presionado el botón derecho sobre la primera nota.
2. Sin soltarlo, arrastrar el puntero a través de todas las notas que se desea borrar.
3. Soltar el botón derecho al terminar.
4. Pulsar `Ctrl+Z` una sola vez para restaurar todas las notas eliminadas durante ese recorrido.

Un clic derecho en una zona vacía no elimina otras notas ni abre una operación destructiva.

### 9.8 Eliminar una selección

1. Seleccionar una o varias notas.
2. Pulsar `Delete` o `Backspace`.
3. Pulsar `Ctrl+Z` para restaurarlas si es necesario.

### 9.9 Copiar, cortar, pegar y duplicar notas

1. Seleccionar las notas.
2. Usar:
   - `Ctrl+C` para copiar.
   - `Ctrl+X` para cortar.
   - `Ctrl+V` para pegar el grupo en el siguiente compás libre.
   - `Ctrl+D` para duplicar el patrón.
3. Verificar que acordes, distancias relativas, duración y velocidad se conservaron.
4. Pulsar `Ctrl+V` de nuevo para pegar en el siguiente compás libre posterior.
5. Si el destino supera el final original, comprobar que el clip MIDI se amplía automáticamente.

Estos atajos son contextuales: si el puntero y foco están fuera del Piano Roll, pueden actuar sobre clips de la ventana Edit.

### 9.10 Cuantizar

1. Seleccionar las notas.
2. Elegir la rejilla, por ejemplo `1/16 note`.
3. Pulsar **Quantize**.
4. Escuchar el resultado.
5. Usar `Ctrl+Z` si la interpretación perdió el feel deseado.

### 9.11 Editar velocidad

1. Seleccionar una nota.
2. En el carril **Velocity**, arrastrar su barra o punto.
3. Usar **Vel +10** o **Vel −10** para cambios uniformes.
4. Reproducir para comprobar la respuesta del instrumento.

### 9.12 Aplicar Legato

1. Seleccionar notas consecutivas.
2. Pulsar **Legato**.
3. Confirmar que sus finales se extienden hacia las notas siguientes sin cambiar el pitch.

### 9.13 Zoom y desplazamiento

1. Pulsar `+` o `−` en el Piano Roll para zoom horizontal.
2. Pulsar **Fit** para encajar el clip.
3. Usar `Ctrl + rueda` alrededor del cursor para zoom temporal.
4. Usar `Alt + rueda` para cambiar la altura de las filas.
5. Usar la rueda normal para desplazamiento vertical.
6. Usar `Shift + rueda` para desplazamiento horizontal.
7. Arrastrar con el botón central para mover la vista en ambas direcciones.

### 9.14 Mostrar nombres de notas

1. Abrir **Labels** en el Piano Roll.
2. Elegir:
   - **Off** para ocultar nombres.
   - **C / Do only** para una referencia por octava.
   - **Every key** para etiquetar todas las teclas.
3. Elegir **English (C, D, E)** o **Español (Do, Re, Mi)**.
4. Elegir **Sharps (♯)**, **Flats (♭)** o **Auto**.
5. Ajustar el tamaño/zoom si las etiquetas quedan demasiado juntas.

La convención actual muestra MIDI 60 como `C3` o `Do3`. Cambiar etiquetas no cambia el pitch ni el número MIDI.

## 10. Pistas y mezcla

### 10.1 Seleccionar varias pistas

1. Hacer clic en la primera cabecera.
2. Usar `Shift+clic` en otra pista para seleccionar un intervalo consecutivo.
3. Usar `Ctrl+clic` para añadir o retirar pistas independientes.

### 10.2 Duplicar pistas — `Alt+Shift+D`

1. Seleccionar las pistas.
2. Pulsar `Alt+Shift+D`.
3. Confirmar que aparecen copias con playlists, automatización, inserts y sends.

### 10.3 Agrupar pistas — `Ctrl+G`

1. Seleccionar dos o más pistas.
2. Pulsar `Ctrl+G`.
3. Escribir el nombre del grupo.
4. Elegir si el grupo enlaza edición, mezcla o ambas.
5. Revisar sus miembros y confirmar.

### 10.4 Crear una carpeta con pistas — `Ctrl+Alt+Shift+N`

1. Seleccionar las pistas que deben quedar dentro de la carpeta.
2. Pulsar `Ctrl+Alt+Shift+N`.
3. Escribir el nombre de la carpeta cuando la interfaz lo solicite.
4. Confirmar que las pistas seleccionadas quedan asociadas a la nueva carpeta.

### 10.5 Commit de pista — `Alt+Shift+C`

1. Guardar la sesión.
2. Seleccionar la pista que se quiere renderizar.
3. Pulsar `Alt+Shift+C`.
4. Esperar el procesamiento.
5. Confirmar la pista de audio resultante y compararla con la original.

### 10.6 Bounce de pista — `Ctrl+Alt+Shift+B`

1. Seleccionar una o más pistas.
2. Pulsar `Ctrl+Alt+Shift+B`.
3. Esperar el render.
4. Revisar la pista resultante antes de desactivar o eliminar las fuentes.

En el estado actual, Commit y Bounce de pista comparten el render interno. No asumir diferencias avanzadas de opciones hasta que la interfaz las exponga.

### 10.7 Guardar un preset de pista — `Alt+Shift+P`

1. Seleccionar la pista de origen.
2. Pulsar `Alt+Shift+P`.
3. Elegir la ruta y escribir un nombre de archivo reconocible.
4. Confirmar el guardado.

El preset guarda tipo, anchura, mezcla, inserts, sends e instrumento de la primera pista seleccionada. No sustituye una copia completa de la sesión.

### 10.8 Bypass de plugins por categoría

1. Seleccionar una o más pistas.
2. Pulsar:
   - `Shift+E` para EQ.
   - `Shift+C` para Dynamics/compresores.
   - `Shift+V` para Reverb.
   - `Shift+D` para Delay.
3. Reproducir para comparar el estado activo y bypass.
4. Repetir el atajo para alternar el estado.

Solo se afectan plugins que declaran la categoría correspondiente. Un plugin sin categoría reconocible no se cambia artificialmente.

## 11. Gestos de ratón y modificadores

### 11.1 Ajuste fino

1. Localizar el control de clip gain.
2. Mantener `Ctrl` mientras se arrastra.
3. Hacer un movimiento pequeño y comprobar el ajuste fino.

Este gesto está confirmado para clip gain. Otros faders todavía no usan una política uniforme.

### 11.2 Doble clic

1. Con Selector, hacer doble clic sobre un clip para seleccionar el clip completo.
2. Con Grabber, hacer doble clic sobre un clip MIDI para abrir el Piano Roll.

### 11.3 Gestos todavía no globales

- `Alt+clic` no aplica una misma regla a todos los controles.
- `Alt+Shift+clic` no está verificado como operación global sobre todas las pistas seleccionadas.
- El triple clic para seleccionar una pista completa está pendiente.

Usar en su lugar selección explícita de pistas, clips o rangos antes de modificar parámetros.

## 12. Cambiar el idioma y el tema

### 12.1 Cambiar el idioma de la interfaz

1. Abrir **Window → Languages & Themes**. También se llega desde **Setup → Languages & Themes**.
2. Buscar **Interface language**.
3. Elegir uno de estos idiomas:
   - English.
   - Español.
   - Português.
   - Français.
   - Deutsch.
   - Русский.
   - 日本語.
4. Confirmar el cambio inmediato en los menús principales, Dashboard, Languages & Themes y Piano Roll. La ventana debe conservar exactamente su posición al cambiar de idioma.
5. Cerrar SoundCraft normalmente.
6. Volver a abrirlo y confirmar que la preferencia persiste.

La localización es incremental. Si una cadena aún no está traducida, se muestra en inglés; no significa que la acción esté deshabilitada.

### 12.2 Elegir un tema incluido

1. En **Window → Languages & Themes**, abrir **Theme**.
2. Elegir una opción:
   - **Studio Dark**: tema oscuro predeterminado.
   - **Studio Light**: tema claro.
   - **Nord Studio**: azul grisáceo de baja saturación.
   - **Dracula Studio**: oscuro con acentos contenidos.
   - **Midnight Cyber**: oscuro azul petróleo, original de SoundCraft.
   - **Follow System**: sigue el modo claro u oscuro del sistema operativo.
3. Trabajar unos minutos y comprobar que textos, notas, rejilla, medidores y estados seleccionados siguen siendo legibles.
4. Cerrar y abrir SoundCraft para confirmar que el tema persiste.

### 12.3 Importar un tema JSON

1. Usar como plantilla [`docs/examples/theme-midnight-ocean.json`](../examples/theme-midnight-ocean.json).
2. Mantener `version` en `1`, elegir un nombre y definir todos los colores como `#RRGGBB`.
3. Abrir **Window → Languages & Themes** y pulsar **Import JSON…**.
4. Seleccionar el archivo `.json`. SoundCraft valida tamaño, estructura, colores y contraste mínimo del texto antes de activarlo.
5. Si el archivo es válido, el nombre personalizado aparece en **Theme** y queda seleccionado.
6. Si falla, leer el mensaje de estado, corregir el JSON y volver a importarlo; el tema anterior permanece activo.

### 12.4 Cambiar la notación musical sin cambiar la interfaz

1. Abrir el Piano Roll.
2. Abrir **Labels**.
3. Elegir inglés `C, D, E…` o español `Do, Re, Mi…`.
4. Elegir sostenidos o bemoles.

La notación del teclado es independiente del idioma general de la interfaz.

## 13. Teclado numérico: estado actual y alternativas

Los accesos `Num 0…9` en modo Transport están **pendientes** porque la capa de entrada actual no distingue de forma fiable el teclado numérico de la fila de números. No se deben usar como si estuvieran implementados.

Alternativas disponibles:

| Acción deseada | Alternativa actual |
|---|---|
| Reproducir/detener | `Espacio` |
| Grabar | `Ctrl+Espacio` o `F12` |
| Loop playback | `Ctrl+Shift+L` |
| Loop record | **Options → Loop Record** o `Alt+L` |
| QuickPunch | **Options → QuickPunch** o `Ctrl+Shift+P` |
| Metrónomo | Control Click/metrónomo de la barra |
| Cuenta previa | **Setup → Click/Countoff…** |
| Rebobinar/avanzar con Num 1/2 | Pendiente como modo numpad |
| MIDI Merge/Replace con Num 9 | Pendiente |

## 14. Flujo completo recomendado: crear una canción y exportarla

1. Crear una sesión con `Ctrl+N`.
2. Guardarla con `Ctrl+S`.
3. Crear una pista Instrument con `Ctrl+Shift+N`.
4. Crear o abrir un clip MIDI.
5. Abrir el Piano Roll con doble clic.
6. Elegir **Size → Large**.
7. Crear notas con `F10` y corregirlas con Selector.
8. Eliminar errores con clic derecho y recuperar con `Ctrl+Z` cuando sea necesario.
9. Crear una pista Audio o importar una muestra con `Ctrl+Shift+I`.
10. Organizar clips con Grid (`F4`) y Snap (`Shift+F4`).
11. Ajustar niveles y plugins en Mix.
12. Guardar con `Ctrl+S`.
13. Escuchar la canción completa desde el inicio.
14. Exportar con `Ctrl+Alt+B`.
15. Reproducir el archivo exportado en otro reproductor.

## 15. Funciones solicitadas que aún no deben considerarse disponibles

| Función | Estado | Qué usar mientras tanto |
|---|---|---|
| `Ctrl+Win+S` para Guardar como | No portable | `Ctrl+Shift+S` |
| `Ctrl+Shift+Espacio` para grabar a media velocidad | Pendiente | Grabación normal con `Ctrl+Espacio`/`F12` |
| `H` en Commands Keyboard Focus | Pendiente | Alinear manualmente con Grid/Spot |
| Transporte con `Num 0…9` | Pendiente | Atajos de transporte de la tabla anterior |
| `Alt+clic` como regla global | Parcial | Seleccionar explícitamente el destino |
| `Alt+Shift+clic` como regla global | No verificado | Seleccionar pistas y cambiar el parámetro |
| Triple clic para pista completa | Pendiente | `Shift+clic` entre cabeceras |
| Traducción total de todos los diálogos | Parcial | Las cadenas pendientes aparecen en inglés |

## 16. Qué revisar si una acción no funciona

1. Confirmar que la ventana correcta tiene el foco.
2. Salir de cualquier campo de texto con `Esc` o haciendo clic en la línea de tiempo.
3. Verificar que existe una pista, clip o rango seleccionado.
4. Comprobar que el comando no aparece deshabilitado en el menú.
5. Para audio, revisar dispositivo, entrada, salida, armado y monitoreo.
6. Para MIDI, comprobar que la pista tiene instrumento o ruta de salida.
7. Para bypass por categoría, comprobar que el plugin declara esa categoría.
8. Deshacer con `Ctrl+Z` si el resultado no era el esperado.
9. Guardar una copia antes de repetir una operación destructiva.
10. Registrar el nombre de la acción, la selección existente y el mensaje mostrado para poder reproducir el problema.

Para una validación formal de estas funciones, usar también [16-end-user-validation-guide.md](16-end-user-validation-guide.md).
