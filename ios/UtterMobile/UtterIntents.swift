import AppIntents
import Foundation

/// Action Button, Back Tap, Siri, and Shortcuts entry point.
/// Conforms to `AudioRecordingIntent` + `LiveActivityIntent` so iOS permits
/// starting or stopping background microphone dictation with a Live Activity.
nonisolated struct DictateWithUtterIntent: AudioRecordingIntent, LiveActivityIntent {
    static let title: LocalizedStringResource = "Dictate with Utter"
    static let description = IntentDescription(
        "Starts recording on first press, then transcribes locally and copies to the clipboard on second press."
    )

    init() {}

    func perform() async throws -> some IntentResult & ReturnsValue<String> {
        let output = await UtterSystemCommandBus.shared.dispatch(.toggleDictation) ?? ""
        return .result(value: output)
    }
}

nonisolated struct WarmUtterKeyboardIntent: AppIntent {
    static let title: LocalizedStringResource = "Warm Utter Keyboard"
    static let description = IntentDescription(
        "Starts Utter's warm microphone session so the keyboard can dictate without switching apps."
    )
    @available(iOS 26.0, *)
    static let supportedModes: IntentModes = .foreground

    init() {}

    func perform() async throws -> some IntentResult {
        await UtterSystemCommandBus.shared.dispatch(.warmSession)
        return .result()
    }
}

nonisolated struct UtterShortcutsProvider: AppShortcutsProvider {
    static var appShortcuts: [AppShortcut] {
        AppShortcut(
            intent: DictateWithUtterIntent(),
            phrases: [
                "Dictate with \(.applicationName)",
                "Start \(.applicationName) dictation",
            ],
            shortTitle: "Dictate",
            systemImageName: "mic.fill"
        )
        AppShortcut(
            intent: WarmUtterKeyboardIntent(),
            phrases: [
                "Warm \(.applicationName) keyboard",
                "Start \(.applicationName) session",
            ],
            shortTitle: "Warm keyboard",
            systemImageName: "keyboard"
        )
    }
}
