import AppIntents
import SwiftUI
import WidgetKit

@available(iOS 18.0, *)
struct UtterDictationControl: ControlWidget {
    static let kind = "app.utter.mobile.control.dictate"

    var body: some ControlWidgetConfiguration {
        StaticControlConfiguration(kind: Self.kind) {
            ControlWidgetButton(action: ToggleUtterControlIntent()) {
                Label("Dictate", systemImage: "mic.fill")
                Text("Utter")
            }
        }
        .displayName("Dictate with Utter")
        .description("Starts or finishes on-device dictation and copies the text.")
    }
}
