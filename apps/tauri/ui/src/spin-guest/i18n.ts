const en = {
  "audio.tap_to_start": "Tap “Listen here” to start audio.",
  "audio.play_failed": "Could not play audio. Tap “Listen here” to retry.",
  "audio.load_failed": "Could not load audio. Tap “Listen here” to retry.",
  "control.failed": "Could not send command",
  "guest.one": "guest",
  "guest.many": "guests",
  "tab.sections": "SPIN sections",
  "tab.search": "Search",
  "tab.queue": "Queue ({count})",
  "toast.added": "Added to queue: {title}",
  "toast.close": "Close notification",
  "now.label": "Now playing",
  "now.playing": "Playing",
  "now.paused": "Paused",
  "now.idle_title": "Nothing playing",
  "now.idle_sub": "Search for a song to get started",
  "now.prev": "Previous",
  "now.pause": "Pause",
  "now.play": "Play",
  "now.next": "Next",
  "now.mute_here": "Mute here",
  "now.listen_here": "Listen here",
  "now.local_volume": "Local volume",
  "queue.aria": "Playback queue",
  "queue.header": "Queue ({count})",
  "queue.empty": "Queue is empty",
  "queue.move_up": "Move up",
  "queue.move_down": "Move down",
  "search.aria": "Search songs",
  "search.header": "Add to queue",
  "search.placeholder": "Search song, artist or album…",
  "search.input_aria": "Search song on YouTube Music",
  "search.clear": "Clear search",
  "search.submit": "Search",
  "search.error": "Error searching songs",
  "search.send_error": "Could not send song",
  "search.added": "Added",
  "search.add": "Add",
  "search.no_results": "No songs found",
  "search.empty_hint": "Type a song or artist name to add it to the queue.",
} as const;

type Key = keyof typeof en;

const es: Record<Key, string> = {
  "audio.tap_to_start": "Toca «Escuchar aquí» para iniciar el audio.",
  "audio.play_failed": "No se pudo reproducir el audio. Toca «Escuchar aquí» para reintentar.",
  "audio.load_failed": "No se pudo cargar el audio. Toca «Escuchar aquí» para reintentar.",
  "control.failed": "No se pudo enviar el comando",
  "guest.one": "invitado",
  "guest.many": "invitados",
  "tab.sections": "Secciones de SPIN",
  "tab.search": "Buscar",
  "tab.queue": "Cola ({count})",
  "toast.added": "Agregada a la cola: {title}",
  "toast.close": "Cerrar notificación",
  "now.label": "Sonando ahora",
  "now.playing": "Reproduciendo",
  "now.paused": "En pausa",
  "now.idle_title": "Sin reproducción",
  "now.idle_sub": "Busca una canción para empezar",
  "now.prev": "Anterior",
  "now.pause": "Pausar",
  "now.play": "Reproducir",
  "now.next": "Siguiente",
  "now.mute_here": "Silenciar aquí",
  "now.listen_here": "Escuchar aquí",
  "now.local_volume": "Volumen local",
  "queue.aria": "Cola de reproducción",
  "queue.header": "Cola ({count})",
  "queue.empty": "La cola está vacía",
  "queue.move_up": "Subir",
  "queue.move_down": "Bajar",
  "search.aria": "Buscar canciones",
  "search.header": "Agregar a la cola",
  "search.placeholder": "Buscar canción, artista o álbum…",
  "search.input_aria": "Buscar canción en YouTube Music",
  "search.clear": "Limpiar búsqueda",
  "search.submit": "Buscar",
  "search.error": "Error al buscar canciones",
  "search.send_error": "No se pudo enviar la canción",
  "search.added": "Agregada",
  "search.add": "Agregar",
  "search.no_results": "No se encontraron canciones",
  "search.empty_hint": "Escribe el nombre de una canción o artista para agregarla a la cola.",
};

const isSpanish =
  typeof navigator !== "undefined" && (navigator.language || "").toLowerCase().startsWith("es");

const dict = isSpanish ? es : en;

export function sg(key: Key, params?: Record<string, string | number>): string {
  let text: string = dict[key] ?? en[key] ?? key;
  if (params) {
    for (const [k, v] of Object.entries(params)) {
      text = text.replaceAll(`{${k}}`, String(v));
    }
  }
  return text;
}
