import init, {on_tab_loaded} from './pkg/dictionary.js'

init("./pkg/dictionary_bg.wasm");

chrome.runtime.onMessage.addListener(function (request, sender, sendResponse) {
    if (request.from_content_script === "loaded") {
        on_tab_loaded(
            sender.tab.url,
            request.from_content_script_word.trim(),
            request.from_content_script_class.trim(),
            request.from_content_script_title.trim()
        );
    }
});