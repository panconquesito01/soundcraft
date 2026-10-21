# SoundCraft — Guía de validación como usuario final

**Objetivo:** comprobar los cambios usando SoundCraft como una persona que compone o edita música, sin revisar código.
**Sesión recomendada:** demo `Midnight Groove`.
**Plataforma principal de esta revisión:** Windows.

Para aprender a ejecutar cada operación, consultar primero [18-end-user-step-by-step.md](18-end-user-step-by-step.md). Este archivo se concentra en validar el resultado, no en reemplazar el manual de uso.

## 1. Abrir SoundCraft para la prueba

Desde la carpeta del proyecto, iniciar la aplicación con la sesión demo:

```powershell
cargo run --release -p soundcraft -- --demo
```

Esperar hasta ver:

- La ventana principal de SoundCraft.
- Las pistas `Kick`, `Snare`, `Hats`, `Bass`, `Pad`, `Keys`, `Lead`, `Verb` y `Master 1`.
- La línea de tiempo y los controles de transporte.

Si aparece un error de dispositivo de audio, se pueden revisar la interfaz y el Piano Roll, pero no se debe marcar como aprobada ninguna prueba de reproducción, grabación o audición.

## 2. Preparar el Piano Roll

1. En la lista izquierda, seleccionar la pista `Keys` o `Lead`.
2. Localizar uno de sus clips MIDI en la línea de tiempo.
3. Con la herramienta Grabber, hacer doble clic sobre el clip.
4. Si el editor no aparece, abrir **Window → MIDI Editor**.
5. Confirmar que en el panel inferior aparecen:
   - El teclado vertical.
   - Las notas MIDI.
   - La zona inferior `Velocity`.
   - Los botones `Quantize`, `Vel +10`, `Vel −10`, `Legato`, `Labels`, `−`, `Fit` y `+`.

La prueba falla si el editor está vacío después de seleccionar un clip MIDI válido o si las notas no coinciden verticalmente con las teclas.

## 3. Nombres de las teclas

### 3.1 Activar y desactivar etiquetas

1. Pulsar `Labels` en la esquina superior derecha del Piano Roll.
2. Elegir `Off`.
3. Confirmar que desaparecen todos los nombres, pero las teclas y notas permanecen iguales.
4. Volver a `Labels` y elegir `C / Do only`.
5. Confirmar que solo se etiqueta una tecla por octava.
6. Elegir `Every key`.
7. Confirmar que todas las teclas visibles muestran un nombre.

### 3.2 Inglés y español

1. En `Labels`, elegir `Every key`.
2. Elegir `English (C, D, E)`.
3. Verificar nombres como `C3`, `D3`, `E3`.
4. Cambiar a `Español (Do, Re, Mi)`.
5. Verificar nombres como `Do3`, `Re3`, `Mi3`.

### 3.3 Sostenidos y bemoles

1. Con las etiquetas en inglés, elegir `Sharps (♯)`.
2. Confirmar que la tecla situada un semitono sobre C aparece como `C♯`.
3. Elegir `Flats (♭)`.
4. Confirmar que la misma tecla aparece como `D♭`.
5. Cambiar al español y repetir: debe verse `Do♯` o `Re♭`.
6. Probar `Auto`; actualmente conserva una representación segura, pero todavía no debe considerarse sensible a la tonalidad del proyecto.

### 3.4 Correspondencia MIDI y octavas

1. Buscar la tecla `C3` o `Do3`.
2. Confirmar que no cambia de posición al alternar idioma o alteraciones.
3. Crear una nota sobre esa fila.
4. Cambiar varias veces entre inglés/español y sostenido/bemol.
5. Confirmar que la nota nunca se mueve y su sonido/pitch no cambia.

Convención actual que debe observarse: MIDI 60 aparece como `C3` o `Do3`.

### 3.5 Persistencia

1. Elegir `Every key`, `Español` y `Flats`.
2. Cerrar SoundCraft normalmente.
3. Volver a abrirlo con la sesión demo.
4. Abrir de nuevo el Piano Roll.
5. Confirmar que siguen seleccionados `Every key`, `Español` y `Flats`.

## 4. Zoom y navegación del Piano Roll

### 4.0 Cambiar el tamaño del editor

1. Pulsar `Size` en la esquina superior derecha del Piano Roll.
2. Elegir `Compact`, `Comfortable`, `Large` y `Maximum`, uno por uno.
3. Confirmar que el panel cambia de altura sin mover ni modificar las notas.
4. Mover el slider `Piano Roll height` y comprobar que el ajuste es continuo.
5. Cerrar y volver a abrir SoundCraft; confirmar que la distribución elegida se conserva.
6. Como alternativa, arrastrar verticalmente el separador sobre el Piano Roll.

La prueba falla si el panel tapa completamente el área de edición, si desaparece el carril de velocidad o si las notas cambian de pitch/tiempo al redimensionar.

### 4.1 Botones de zoom

1. Pulsar `Fit`.
2. Confirmar que el clip completo cabe horizontalmente.
3. Pulsar `+` varias veces.
4. Confirmar que las notas se ensanchan y se ve menos tiempo.
5. Pulsar `−` varias veces.
6. Confirmar que se vuelve a ver más tiempo.

La altura musical de las notas no debe cambiar con estos tres botones.

### 4.2 Zoom alrededor del cursor

1. Colocar el cursor sobre una nota concreta.
2. Usar el gesto de zoom del touchpad o `Ctrl + rueda`.
3. Confirmar que el punto temporal situado bajo el cursor permanece prácticamente en el mismo lugar.
4. Repetir acercando y alejando.

### 4.3 Zoom vertical

1. Colocar el cursor dentro de la cuadrícula.
2. Mantener `Alt` y usar la rueda vertical.
3. Confirmar que aumenta o disminuye la altura de las filas.
4. Verificar que las notas siguen alineadas con sus teclas.
5. Con `Every key`, confirmar que las etiquetas siguen siendo legibles hasta el límite razonable del zoom.

### 4.4 Desplazamiento

1. Usar la rueda sin modificadores: debe desplazarse verticalmente por las notas.
2. Mantener `Shift` y usar la rueda: debe desplazarse horizontalmente.
3. Arrastrar con el botón central: debe mover la vista en ambas direcciones.
4. Intentar desplazarse antes del inicio o mucho después del final del clip.
5. Confirmar que la vista no se pierde fuera de los límites útiles.

## 5. Selección y edición de notas

### 5.1 Selección individual y múltiple

1. Hacer clic en una nota: debe quedar seleccionada en blanco.
2. Mantener `Shift` y hacer clic en otra nota.
3. Confirmar que ambas quedan seleccionadas.
4. Arrastrar desde un espacio vacío formando un rectángulo alrededor de varias notas.
5. Confirmar que solo se seleccionan las notas tocadas por el rectángulo.
6. Hacer clic en un espacio vacío para limpiar la selección.

### 5.2 Crear notas

1. Seleccionar la herramienta Pencil en la barra superior.
2. Hacer clic en una celda vacía del Piano Roll.
3. Confirmar que se crea una nota alineada con la rejilla.
4. Crear notas en filas diferentes.
5. Confirmar que cada nota aparece frente a la tecla correcta.

### 5.3 Mover y transponer

1. Seleccionar una nota y arrastrarla horizontalmente.
2. Confirmar que se mueve por pasos de la rejilla.
3. Arrastrarla verticalmente.
4. Confirmar que cambia por semitonos y no sale del rango MIDI.
5. Seleccionar una o varias notas.
6. Pulsar flecha arriba/abajo: deben transponerse un semitono.
7. Pulsar `Shift + arriba/abajo`: deben transponerse una octava.
8. Pulsar izquierda/derecha: deben moverse un paso de la rejilla.

### 5.4 Cambiar duración

1. Colocar el puntero cerca del borde derecho de una nota.
2. Confirmar que aparece el cursor horizontal de doble flecha y arrastrar el borde.
3. Confirmar que cambia la duración, no la posición ni el pitch.
4. Intentar reducirla al mínimo.
5. Confirmar que no llega a duración cero ni negativa.
6. Seleccionar varias notas de duraciones diferentes.
7. Arrastrar el borde derecho de una de ellas y confirmar que todas cambian por el mismo desplazamiento, conservando inicio, pitch y velocidad.
8. Confirmar que el extremo se ajusta a la rejilla.
9. Repetir manteniendo `Alt` y confirmar que permite un ajuste temporal libre.
10. Confirmar que todas las notas siguen seleccionadas después de soltar el ratón.
11. Pulsar `Ctrl+Z` una vez y confirmar que todas recuperan sus duraciones anteriores.

### 5.5 Eliminar y deshacer

1. Seleccionar varias notas.
2. Pulsar `Delete` o `Backspace`.
3. Confirmar que desaparecen.
4. Pulsar `Ctrl+Z`.
5. Confirmar que todas reaparecen en su posición original.

### 5.6 Eliminar directamente con clic derecho

1. Dejar varias notas visibles, sin necesidad de seleccionarlas.
2. Hacer clic derecho exactamente sobre una nota.
3. Confirmar que desaparece solo la nota apuntada.
4. Hacer clic derecho en una zona vacía y confirmar que no se elimina nada.
5. Pulsar `Ctrl+Z` y comprobar que la nota vuelve a su posición, duración, velocidad y pitch originales.

### 5.7 Borrar varias notas arrastrando el clic derecho

1. Crear o dejar visibles al menos tres notas separadas.
2. Mantener presionado el botón derecho sobre la primera y atravesar las demás sin soltarlo.
3. Confirmar que desaparece cada nota cruzada, incluso si el puntero se mueve rápido entre dos fotogramas.
4. Confirmar que una nota cercana que no fue cruzada permanece.
5. Soltar el botón y pulsar `Ctrl+Z` una sola vez.
6. Confirmar que reaparecen juntas todas las notas eliminadas durante el gesto.

### 5.8 Cambiar el idioma de la interfaz

1. Abrir **Window → Languages & Themes**.
2. En `Interface language`, elegir `Español`.
3. Confirmar que cambian inmediatamente los menús principales, Dashboard, Languages & Themes y los controles principales del Piano Roll.
4. Confirmar que la ventana conserva su posición exacta; el cambio de título traducido no debe centrarla ni desplazarla.
5. Repetir con `Português`, `Français`, `Deutsch`, `Русский` y `日本語`.
6. En japonés, confirmar que no aparecen cuadros vacíos en lugar de caracteres. Si aparecen, registrar sistema operativo y fuentes instaladas.
7. Probar `Studio Dark`, `Studio Light`, `Nord Studio`, `Dracula Studio`, `Midnight Cyber` y `Follow System`.
8. Importar [`docs/examples/theme-midnight-ocean.json`](../examples/theme-midnight-ocean.json) y confirmar que aparece por su nombre en `Theme`.
9. Cerrar y volver a abrir SoundCraft; confirmar que idioma y tema se conservan.
10. Revisar una pantalla aún no migrada: debe mostrar inglés legible, nunca una clave técnica ni texto vacío.

Esta prueba valida la base y las superficies migradas, no una traducción completa de toda la aplicación.

## 6. Copiar, cortar, pegar y duplicar notas

### 6.1 Copiar y pegar

1. Seleccionar un pequeño patrón de varias notas.
2. Pulsar `Ctrl+C`.
3. Sin retirar el foco del Piano Roll, pulsar `Ctrl+V`.
4. Confirmar que el patrón aparece completo en el siguiente compás libre. Si el compás inmediato ya contiene notas, SoundCraft debe saltarlo sin superponer el patrón.
5. Pulsar `Ctrl+V` otra vez y confirmar que la nueva copia avanza al siguiente compás libre.
6. Verificar que conserva:
   - Distancias relativas.
   - Acordes.
   - Duraciones.
   - Velocidades.
7. Si el destino queda fuera de la duración original del clip, confirmar que el clip MIDI se amplía para contener las notas.
8. Pulsar `Ctrl+Z` una vez.
9. Confirmar que desaparece todo el último pegado y se restaura la duración anterior del clip, no solo una nota.

### 6.2 Cortar

1. Seleccionar varias notas.
2. Pulsar `Ctrl+X`.
3. Confirmar que desaparecen.
4. Pulsar `Ctrl+Z`.
5. Confirmar que todas reaparecen juntas.

### 6.3 Duplicar

1. Seleccionar un patrón.
2. Pulsar `Ctrl+D`.
3. Confirmar que aparece una copia después del intervalo ocupado por la selección.
4. Confirmar que el patrón original permanece.
5. Pulsar `Ctrl+Z` y comprobar que solo se elimina la copia.

### 6.4 Contexto correcto

1. Mover el cursor fuera del Piano Roll.
2. Seleccionar un clip normal en la línea de tiempo.
3. Usar `Ctrl+C` o `Ctrl+D`.
4. Confirmar que el atajo vuelve a actuar sobre la edición principal, no sobre notas antiguas del Piano Roll.

### 6.5 Copiar y pegar clips desde el menú contextual

1. Elegir Grabber y hacer clic derecho sobre un clip de audio o MIDI.
2. Elegir **Copy Clip**.
3. Hacer clic derecho en un punto vacío de una pista compatible, en una posición temporal distinta.
4. Elegir **Paste Here**.
5. Confirmar que aparece una copia exactamente en la pista y posición señaladas y que el original no cambia.
6. Pulsar `Ctrl+Z` una vez y confirmar que desaparece únicamente la copia.
7. Repetir con **Cut Clip** y confirmar que el portapapeles conserva el material para pegarlo.

### 6.6 Copiar y pegar un clip con teclado

1. En la ventana Edit, hacer clic una vez sobre un clip MIDI para seleccionarlo.
2. Pulsar `Ctrl+C`.
3. Sin hacer clic en otra posición, pulsar `Ctrl+V`.
4. Confirmar que la copia aparece después del clip seleccionado, en el primer intervalo libre de la pista, y no encima del original.
5. Confirmar que el clip recién pegado queda como el rango de edición activo y que la selección antigua no intercepta el siguiente comando.
6. Pulsar `Ctrl+Z` una vez y confirmar que desaparece únicamente la copia.
7. Repetir la prueba con un clip de audio.

## 7. Velocidad, cuantización y legato

### 7.1 Velocidad

1. Seleccionar una nota.
2. En la franja `Velocity`, arrastrar el punto/barra correspondiente.
3. Confirmar que cambia su altura.
4. Pulsar `Vel +10` y después `Vel −10`.
5. Confirmar que la velocidad se mantiene entre 1 y 127.
6. Deshacer y confirmar que vuelve al valor anterior.

### 7.2 Cuantización

1. Mover una nota ligeramente fuera de la rejilla.
2. Elegir una rejilla visible, por ejemplo `1/16 note`.
3. Pulsar `Quantize`.
4. Confirmar que la nota se ajusta a la rejilla seleccionada.
5. Deshacer y confirmar que vuelve a la posición anterior.

### 7.3 Legato

1. Usar un patrón con notas consecutivas y espacios entre ellas.
2. Pulsar `Legato`.
3. Confirmar que las duraciones se extienden hacia las notas siguientes sin cambiar sus pitches.
4. Deshacer y comprobar la restauración completa.

## 8. Rendimiento con muchas notas

1. Abrir un clip MIDI denso o duplicar un patrón varias veces.
2. Hacer zoom hasta ver una zona pequeña.
3. Desplazarse rápidamente a izquierda/derecha y arriba/abajo.
4. Seleccionar, mover y redimensionar notas visibles.
5. Confirmar que:
   - No desaparecen notas largas que atraviesan el borde de la vista.
   - No se seleccionan notas invisibles al hacer clic.
   - No hay congelamientos prolongados.
   - El cursor responde de forma continua.

Esta es una comprobación perceptiva. No demuestra todavía objetivos profesionales de rendimiento; hacen falta métricas antes/después para cerrar SC-014.

## 9. Interfaz en ventanas pequeñas

1. Cerrar el Piano Roll temporalmente si hace falta y reducir progresivamente el ancho de la ventana.
2. Confirmar que siempre permanecen visibles:
   - Stop.
   - Play.
   - Record.
   - Contador de posición.
3. En un ancho pequeño, confirmar que aparece el botón `More`.
4. Abrir `More` y verificar acceso a:
   - Edit mode.
   - Tool.
   - Zoom.
   - Loop playback.
   - Grid lines.
   - Click y demás opciones mostradas.
5. Reducir más la ventana.
6. Confirmar que la barra de menús se convierte en `Menu` antes de chocar con el título de la ventana.
7. Volver a maximizar.
8. Confirmar que reaparecen los controles completos sin cerrar ni reiniciar la aplicación.

También conviene repetir con escalado de Windows 100 %, 125 %, 150 % y 200 %. Los paneles laterales todavía no tienen adaptación completa; ese punto debe registrarse como pendiente, no como fallo nuevo de esta entrega.

## 10. Freeze y Renumber Bars

Estas pruebas confirman que SoundCraft ya no promete una operación que todavía no realiza.

### 10.1 Freeze

1. Seleccionar una pista de audio o instrumento que no esté congelada.
2. Abrir el menú `Track`.
3. Localizar `Freeze`.
4. Debe aparecer deshabilitado o explicar que la creación del render congelado todavía no está implementada.
5. Confirmar que la pista no cambia falsamente a estado congelado y que no se crea audio ficticio.

`Commit...` puede aparecer como alternativa destructiva, pero no es equivalente a Freeze y no debe ejecutarse durante esta prueba salvo que se trabaje sobre una copia de la sesión.

### 10.2 Renumber Bars

1. Abrir `Event`.
2. Localizar `Renumber Bars...`.
3. Debe aparecer deshabilitado o informar que el mapeo de números de compás no está implementado.
4. Confirmar que los números de compás y la sesión permanecen sin cambios.

## 11. Guardado, reapertura y rutas de audio en Windows

Usar archivos de prueba, nunca la única copia de una grabación importante.

1. Crear una sesión nueva.
2. Importar un WAV pequeño.
3. Guardar la sesión dentro de una carpeta de prueba.
4. Cerrar SoundCraft normalmente.
5. Volver a abrir la sesión desde la misma ruta.
6. Confirmar que el clip, la forma de onda y el audio están presentes.
7. Reproducir el clip si existe un dispositivo de audio funcional.
8. Guardar, cerrar y abrir una segunda vez.
9. Confirmar que SoundCraft no solicita relocalizar un archivo que sigue exactamente en el mismo lugar.

Prueba adicional segura:

1. Copiar toda la carpeta de la sesión a otra ubicación.
2. Abrir la copia.
3. Confirmar que SoundCraft no sobrescribe el audio perteneciente a la carpeta original.

## 12. Archivos inválidos o demasiado grandes

No es necesario crear manualmente archivos de 1 GB. Como usuario final solo debe comprobarse el comportamiento visible con un archivo claramente inválido:

1. Crear una copia de un archivo de texto sin información sensible.
2. Cambiar su extensión a una extensión que SoundCraft intente importar.
3. Intentar abrir o importar la copia.
4. Confirmar que aparece un error comprensible.
5. Confirmar que SoundCraft sigue abierto y la sesión no cambia.
6. Continuar editando o guardar una copia para confirmar que la aplicación se recuperó.

Los límites extremos, bundles hostiles y archivos que crecen durante la lectura requieren pruebas automatizadas; no son una prueba manual razonable para un usuario final.

## 13. Lista rápida de aprobación

Marcar cada elemento después de probarlo:

- [ ] La sesión demo abre.
- [ ] El Piano Roll abre desde un clip MIDI.
- [ ] `Off`, `C / Do only` y `Every key` funcionan.
- [ ] Inglés y español funcionan.
- [ ] Sostenidos y bemoles representan la misma tecla sin mover notas.
- [ ] Las preferencias sobreviven al reinicio.
- [ ] `−`, `Fit`, `+`, zoom al cursor y zoom vertical funcionan.
- [ ] Scroll vertical, horizontal y paneo funcionan.
- [ ] Selección individual, múltiple y por rectángulo funcionan.
- [ ] Crear, mover y transponer notas funciona.
- [ ] El borde derecho redimensiona una nota o todo el grupo seleccionado, con snap y ajuste libre mediante `Alt`.
- [ ] Mantener el clic derecho y atravesar notas las borra en un solo gesto y un único `Ctrl+Z` las restaura.
- [ ] Copiar, cortar, pegar y duplicar conservan el patrón.
- [ ] **Copy Clip** y **Paste Here** copian un clip a la pista y posición elegidas.
- [ ] Undo/redo revierte cada operación completa.
- [ ] Velocidad, Quantize y Legato funcionan y se pueden deshacer.
- [ ] La interfaz estrecha mantiene transporte y contador.
- [ ] `More` contiene las acciones ocultas.
- [ ] El menú superior se colapsa sin superponerse.
- [ ] Freeze no afirma haber renderizado algo que no existe.
- [ ] Renumber Bars no modifica la sesión.
- [ ] Guardar y reabrir conserva sesión y medios.
- [ ] Un archivo inválido muestra error sin cerrar SoundCraft.

## 14. Cómo registrar un fallo

Por cada fallo, anotar:

1. Nombre de la prueba.
2. Pasos exactos realizados.
3. Qué se esperaba.
4. Qué ocurrió.
5. Captura de pantalla o video corto.
6. Si se reproduce siempre o solo a veces.
7. Resolución, escalado de Windows y dispositivo de audio usado.
8. Archivo o sesión mínima necesaria para reproducirlo, sin información privada.

No marcar como fallo una función que el plan identifica expresamente como pendiente. En ese caso registrar “no implementada” y conservar la expectativa para una etapa posterior.
