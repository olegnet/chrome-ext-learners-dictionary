const SITE_ORIGIN = 'https://www.oxfordlearnersdictionaries.com';
const SIDEPANEL = 'sidepanel.html';

chrome.sidePanel.setPanelBehavior({openPanelOnActionClick: true})
    .catch((error) => {
        // console.error(error);
    });

chrome.tabs.onUpdated.addListener(async (tabId, info, tab) => {
    if (!tab.url) return;

    const url = new URL(tab.url);

    if (url.origin === SITE_ORIGIN) {
        await chrome.sidePanel.setOptions({
            tabId, path: SIDEPANEL, enabled: true
        });
    } else {
        await chrome.sidePanel.setOptions({
            tabId, enabled: false
        });
    }
});

chrome.commands.onCommand.addListener((command) => {
    // console.log(`Command: ${command}`);
});

chrome.runtime.onMessage.addListener(function (request, sender, sendResponse) {
    // for sendMessage in content-script.js
});
