//! Small, typed localization foundation for the UI.
//!
//! The English command catalog remains the stable identifier/source text. UI surfaces ask this
//! module for a display string and safely fall back to English while the full catalog is migrated.

/// User-selectable interface language, persisted with the other UI preferences.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum Language {
    #[default]
    English,
    Spanish,
    Portuguese,
    French,
    German,
    Russian,
    Japanese,
}

impl Language {
    pub const ALL: [Self; 7] = [Self::English, Self::Spanish, Self::Portuguese, Self::French, Self::German, Self::Russian, Self::Japanese];

    pub const fn native_name(self) -> &'static str {
        match self {
            Self::English => "English",
            Self::Spanish => "Español",
            Self::Portuguese => "Português",
            Self::French => "Français",
            Self::German => "Deutsch",
            Self::Russian => "Русский",
            Self::Japanese => "日本語",
        }
    }
}

/// Translate a source-English UI string. Unknown strings deliberately fall back to their source
/// text, so an incomplete translation can never hide a command or produce a blank control.
pub fn text(language: Language, source: &str) -> &str {
    match language {
        Language::English => source,
        Language::Spanish => spanish(source),
        Language::Portuguese => portuguese(source),
        Language::French => french(source),
        Language::German => german(source),
        Language::Russian => russian(source),
        Language::Japanese => japanese(source),
    }
}

macro_rules! catalog {
    ($name:ident { $($source:literal => $translated:literal),* $(,)? }) => {
        fn $name(source: &str) -> &str {
            match source {
                $($source => $translated,)*
                _ => source,
            }
        }
    };
}

catalog!(spanish {
    "File" => "Archivo", "Edit" => "Edición", "View" => "Ver", "Track" => "Pista",
    "Clip" => "Clip", "Event" => "Evento", "Options" => "Opciones", "Setup" => "Configuración",
    "Window" => "Ventana", "Menu" => "Menú", "Dashboard" => "Inicio", "About SoundCraft" => "Acerca de SoundCraft",
    "Session Info" => "Información de la sesión", "Session Audio Health" => "Estado del audio de la sesión", "Quit" => "Salir",
    "Appearance" => "Apariencia", "Languages & Themes" => "Idiomas y temas", "Interface language" => "Idioma de la interfaz", "Interface scale" => "Escala de la interfaz",
    "Reduce motion" => "Reducir movimiento", "Theme" => "Tema", "Auto-update configuration" => "Actualizar configuración automáticamente",
    "Studio Dark" => "Estudio oscuro", "Studio Light" => "Estudio claro", "Follow System" => "Seguir el sistema",
    "Midnight Cyber" => "Cyber medianoche", "Custom theme" => "Tema personalizado", "Import JSON…" => "Importar JSON…",
    "JSON themes are validated for size, colors and readable text contrast." => "Los temas JSON se validan por tamaño, colores y contraste de texto legible.",
    "Reset" => "Restablecer", "Start a session" => "Iniciar una sesión", "New Session…" => "Nueva sesión…",
    "Open Session…" => "Abrir sesión…", "Recent sessions" => "Sesiones recientes", "No recent sessions yet." => "Aún no hay sesiones recientes.",
    "MIDI EDITOR" => "EDITOR MIDI", "Size" => "Tamaño", "Piano Roll height" => "Altura del Piano Roll",
    "Compact" => "Compacto", "Comfortable" => "Cómodo", "Large" => "Grande", "Maximum" => "Máximo",
    "You can also drag the divider above the editor." => "También puede arrastrar el separador sobre el editor.",
    "Resize the Piano Roll or drag its upper divider" => "Cambie el tamaño del Piano Roll o arrastre su separador superior",
    "Labels" => "Etiquetas", "Keyboard labels" => "Etiquetas del teclado", "Off" => "Desactivadas",
    "C / Do only" => "Solo C / Do", "Every key" => "Todas las teclas", "Note names" => "Nombres de notas",
    "Accidentals" => "Alteraciones", "Sharps (♯)" => "Sostenidos (♯)", "Flats (♭)" => "Bemoles (♭)",
    "Auto (key-aware later)" => "Automático (tonalidad próximamente)", "Legato" => "Legato", "Quantize" => "Cuantizar",
    "Fit" => "Ajustar", "More" => "Más"
});

catalog!(portuguese {
    "File" => "Arquivo", "Edit" => "Editar", "View" => "Exibir", "Track" => "Faixa", "Clip" => "Clipe",
    "Event" => "Evento", "Options" => "Opções", "Setup" => "Configuração", "Window" => "Janela", "Menu" => "Menu",
    "Dashboard" => "Início", "About SoundCraft" => "Sobre o SoundCraft", "Session Info" => "Informações da sessão",
    "Session Audio Health" => "Integridade do áudio da sessão", "Quit" => "Sair", "Appearance" => "Aparência", "Languages & Themes" => "Idiomas e temas",
    "Interface language" => "Idioma da interface", "Interface scale" => "Escala da interface", "Reduce motion" => "Reduzir movimento",
    "Studio Dark" => "Estúdio escuro", "Studio Light" => "Estúdio claro", "Follow System" => "Seguir o sistema",
    "Midnight Cyber" => "Cyber meia-noite", "Custom theme" => "Tema personalizado", "Import JSON…" => "Importar JSON…",
    "JSON themes are validated for size, colors and readable text contrast." => "Os temas JSON são validados quanto ao tamanho, às cores e ao contraste legível do texto.",
    "Theme" => "Tema", "Auto-update configuration" => "Atualizar configuração automaticamente", "Reset" => "Redefinir",
    "Start a session" => "Iniciar uma sessão", "New Session…" => "Nova sessão…", "Open Session…" => "Abrir sessão…",
    "Recent sessions" => "Sessões recentes", "No recent sessions yet." => "Ainda não há sessões recentes.",
    "MIDI EDITOR" => "EDITOR MIDI", "Size" => "Tamanho", "Piano Roll height" => "Altura do Piano Roll",
    "Compact" => "Compacto", "Comfortable" => "Confortável", "Large" => "Grande", "Maximum" => "Máximo",
    "You can also drag the divider above the editor." => "Você também pode arrastar o divisor acima do editor.",
    "Resize the Piano Roll or drag its upper divider" => "Redimensione o Piano Roll ou arraste o divisor superior",
    "Labels" => "Rótulos", "Keyboard labels" => "Rótulos do teclado", "Off" => "Desativados", "C / Do only" => "Somente C / Do",
    "Every key" => "Todas as teclas", "Note names" => "Nomes das notas", "Accidentals" => "Acidentes",
    "Sharps (♯)" => "Sustenidos (♯)", "Flats (♭)" => "Bemóis (♭)", "Auto (key-aware later)" => "Automático (tonalidade futuramente)",
    "Legato" => "Legato", "Quantize" => "Quantizar", "Fit" => "Ajustar", "More" => "Mais"
});

catalog!(french {
    "File" => "Fichier", "Edit" => "Édition", "View" => "Affichage", "Track" => "Piste", "Clip" => "Clip",
    "Event" => "Événement", "Options" => "Options", "Setup" => "Configuration", "Window" => "Fenêtre", "Menu" => "Menu",
    "Dashboard" => "Accueil", "About SoundCraft" => "À propos de SoundCraft", "Session Info" => "Informations de session",
    "Session Audio Health" => "État audio de la session", "Quit" => "Quitter", "Appearance" => "Apparence", "Languages & Themes" => "Langues et thèmes",
    "Interface language" => "Langue de l’interface", "Interface scale" => "Échelle de l’interface", "Reduce motion" => "Réduire les animations",
    "Studio Dark" => "Studio sombre", "Studio Light" => "Studio clair", "Follow System" => "Suivre le système",
    "Midnight Cyber" => "Cyber minuit", "Custom theme" => "Thème personnalisé", "Import JSON…" => "Importer un JSON…",
    "JSON themes are validated for size, colors and readable text contrast." => "Les thèmes JSON sont validés pour leur taille, leurs couleurs et un contraste de texte lisible.",
    "Theme" => "Thème", "Auto-update configuration" => "Mise à jour automatique de la configuration", "Reset" => "Réinitialiser",
    "Start a session" => "Démarrer une session", "New Session…" => "Nouvelle session…", "Open Session…" => "Ouvrir une session…",
    "Recent sessions" => "Sessions récentes", "No recent sessions yet." => "Aucune session récente.",
    "MIDI EDITOR" => "ÉDITEUR MIDI", "Size" => "Taille", "Piano Roll height" => "Hauteur du Piano Roll",
    "Compact" => "Compact", "Comfortable" => "Confortable", "Large" => "Grand", "Maximum" => "Maximum",
    "You can also drag the divider above the editor." => "Vous pouvez aussi faire glisser le séparateur au-dessus de l’éditeur.",
    "Resize the Piano Roll or drag its upper divider" => "Redimensionnez le Piano Roll ou faites glisser son séparateur supérieur",
    "Labels" => "Étiquettes", "Keyboard labels" => "Étiquettes du clavier", "Off" => "Désactivées", "C / Do only" => "C / Do uniquement",
    "Every key" => "Toutes les touches", "Note names" => "Noms des notes", "Accidentals" => "Altérations",
    "Sharps (♯)" => "Dièses (♯)", "Flats (♭)" => "Bémols (♭)", "Auto (key-aware later)" => "Automatique (tonalité ultérieurement)",
    "Legato" => "Legato", "Quantize" => "Quantifier", "Fit" => "Ajuster", "More" => "Plus"
});

catalog!(german {
    "File" => "Datei", "Edit" => "Bearbeiten", "View" => "Ansicht", "Track" => "Spur", "Clip" => "Clip",
    "Event" => "Ereignis", "Options" => "Optionen", "Setup" => "Einrichtung", "Window" => "Fenster", "Menu" => "Menü",
    "Dashboard" => "Start", "About SoundCraft" => "Über SoundCraft", "Session Info" => "Sitzungsinformationen",
    "Session Audio Health" => "Audiozustand der Sitzung", "Quit" => "Beenden", "Appearance" => "Darstellung", "Languages & Themes" => "Sprachen und Designs",
    "Interface language" => "Oberflächensprache", "Interface scale" => "Oberflächenskalierung", "Reduce motion" => "Bewegung reduzieren",
    "Studio Dark" => "Studio dunkel", "Studio Light" => "Studio hell", "Follow System" => "Systemeinstellung",
    "Midnight Cyber" => "Cyber-Mitternacht", "Custom theme" => "Benutzerdefiniertes Design", "Import JSON…" => "JSON importieren…",
    "JSON themes are validated for size, colors and readable text contrast." => "JSON-Designs werden auf Größe, Farben und gut lesbaren Textkontrast geprüft.",
    "Theme" => "Design", "Auto-update configuration" => "Konfiguration automatisch aktualisieren", "Reset" => "Zurücksetzen",
    "Start a session" => "Sitzung starten", "New Session…" => "Neue Sitzung…", "Open Session…" => "Sitzung öffnen…",
    "Recent sessions" => "Letzte Sitzungen", "No recent sessions yet." => "Noch keine letzten Sitzungen.",
    "MIDI EDITOR" => "MIDI-EDITOR", "Size" => "Größe", "Piano Roll height" => "Piano-Roll-Höhe",
    "Compact" => "Kompakt", "Comfortable" => "Komfortabel", "Large" => "Groß", "Maximum" => "Maximum",
    "You can also drag the divider above the editor." => "Sie können auch die Trennlinie über dem Editor ziehen.",
    "Resize the Piano Roll or drag its upper divider" => "Piano Roll skalieren oder obere Trennlinie ziehen",
    "Labels" => "Beschriftungen", "Keyboard labels" => "Tastenbeschriftungen", "Off" => "Aus", "C / Do only" => "Nur C / Do",
    "Every key" => "Jede Taste", "Note names" => "Notennamen", "Accidentals" => "Vorzeichen",
    "Sharps (♯)" => "Kreuze (♯)", "Flats (♭)" => "Bs (♭)", "Auto (key-aware later)" => "Automatisch (Tonart später)",
    "Legato" => "Legato", "Quantize" => "Quantisieren", "Fit" => "Einpassen", "More" => "Mehr"
});

catalog!(russian {
    "File" => "Файл", "Edit" => "Правка", "View" => "Вид", "Track" => "Дорожка", "Clip" => "Клип",
    "Event" => "Событие", "Options" => "Параметры", "Setup" => "Настройка", "Window" => "Окно", "Menu" => "Меню",
    "Dashboard" => "Начало", "About SoundCraft" => "О SoundCraft", "Session Info" => "Сведения о сессии",
    "Session Audio Health" => "Состояние аудио сессии", "Quit" => "Выход", "Appearance" => "Оформление", "Languages & Themes" => "Языки и темы",
    "Interface language" => "Язык интерфейса", "Interface scale" => "Масштаб интерфейса", "Reduce motion" => "Уменьшить движение",
    "Studio Dark" => "Тёмная студия", "Studio Light" => "Светлая студия", "Follow System" => "Как в системе",
    "Midnight Cyber" => "Полночный кибер", "Custom theme" => "Пользовательская тема", "Import JSON…" => "Импортировать JSON…",
    "JSON themes are validated for size, colors and readable text contrast." => "Темы JSON проверяются по размеру, цветам и читаемому контрасту текста.",
    "Theme" => "Тема", "Auto-update configuration" => "Автообновление конфигурации", "Reset" => "Сбросить",
    "Start a session" => "Начать сессию", "New Session…" => "Новая сессия…", "Open Session…" => "Открыть сессию…",
    "Recent sessions" => "Недавние сессии", "No recent sessions yet." => "Недавних сессий пока нет.",
    "MIDI EDITOR" => "MIDI-РЕДАКТОР", "Size" => "Размер", "Piano Roll height" => "Высота Piano Roll",
    "Compact" => "Компактно", "Comfortable" => "Удобно", "Large" => "Большой", "Maximum" => "Максимум",
    "You can also drag the divider above the editor." => "Можно также перетащить разделитель над редактором.",
    "Resize the Piano Roll or drag its upper divider" => "Измените размер Piano Roll или перетащите верхний разделитель",
    "Labels" => "Подписи", "Keyboard labels" => "Подписи клавиш", "Off" => "Выкл.", "C / Do only" => "Только C / Do",
    "Every key" => "Все клавиши", "Note names" => "Названия нот", "Accidentals" => "Знаки альтерации",
    "Sharps (♯)" => "Диезы (♯)", "Flats (♭)" => "Бемоли (♭)", "Auto (key-aware later)" => "Авто (тональность позже)",
    "Legato" => "Легато", "Quantize" => "Квантизация", "Fit" => "Вместить", "More" => "Ещё"
});

catalog!(japanese {
    "File" => "ファイル", "Edit" => "編集", "View" => "表示", "Track" => "トラック", "Clip" => "クリップ",
    "Event" => "イベント", "Options" => "オプション", "Setup" => "設定", "Window" => "ウィンドウ", "Menu" => "メニュー",
    "Dashboard" => "スタート", "About SoundCraft" => "SoundCraftについて", "Session Info" => "セッション情報",
    "Session Audio Health" => "セッションのオーディオ状態", "Quit" => "終了", "Appearance" => "外観", "Languages & Themes" => "言語とテーマ",
    "Interface language" => "表示言語", "Interface scale" => "インターフェイスの倍率", "Reduce motion" => "動きを減らす",
    "Studio Dark" => "スタジオ・ダーク", "Studio Light" => "スタジオ・ライト", "Follow System" => "システム設定に従う",
    "Midnight Cyber" => "ミッドナイト・サイバー", "Custom theme" => "カスタムテーマ", "Import JSON…" => "JSONを読み込む…",
    "JSON themes are validated for size, colors and readable text contrast." => "JSONテーマはサイズ、色、読みやすい文字コントラストを検証します。",
    "Theme" => "テーマ", "Auto-update configuration" => "構成を自動更新", "Reset" => "リセット",
    "Start a session" => "セッションを開始", "New Session…" => "新規セッション…", "Open Session…" => "セッションを開く…",
    "Recent sessions" => "最近のセッション", "No recent sessions yet." => "最近のセッションはありません。",
    "MIDI EDITOR" => "MIDIエディター", "Size" => "サイズ", "Piano Roll height" => "ピアノロールの高さ",
    "Compact" => "コンパクト", "Comfortable" => "標準", "Large" => "大", "Maximum" => "最大",
    "You can also drag the divider above the editor." => "エディター上部の区切り線をドラッグすることもできます。",
    "Resize the Piano Roll or drag its upper divider" => "ピアノロールのサイズを変更するか、上部の区切り線をドラッグします",
    "Labels" => "ラベル", "Keyboard labels" => "鍵盤ラベル", "Off" => "オフ", "C / Do only" => "C / Doのみ",
    "Every key" => "すべての鍵盤", "Note names" => "音名", "Accidentals" => "変化記号",
    "Sharps (♯)" => "シャープ (♯)", "Flats (♭)" => "フラット (♭)", "Auto (key-aware later)" => "自動（調号対応は今後）",
    "Legato" => "レガート", "Quantize" => "クオンタイズ", "Fit" => "全体表示", "More" => "その他"
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_selectable_language_translates_the_core_menu() {
        for language in Language::ALL.into_iter().filter(|language| *language != Language::English) {
            assert_ne!(text(language, "File"), "File", "{language:?} has no core translation");
            assert!(!language.native_name().is_empty());
        }
    }

    #[test]
    fn unknown_strings_fall_back_to_english_and_language_round_trips() {
        assert_eq!(text(Language::Japanese, "Uncatalogued command"), "Uncatalogued command");
        for language in Language::ALL {
            let value = serde_json::to_value(language).unwrap_or_default();
            assert_eq!(serde_json::from_value::<Language>(value).unwrap_or_default(), language);
        }
    }
}
