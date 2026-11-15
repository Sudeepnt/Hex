import SwiftUI
import WidgetKit

@main
struct UtterWidgetsBundle: WidgetBundle {
    var body: some Widget {
        UtterDictationLiveActivity()
        if #available(iOS 18.0, *) {
            UtterDictationControl()
        }
    }
}
