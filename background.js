const SITE_ORIGIN = 'https://www.oxfordlearnersdictionaries.com';
const SIDEPANEL = 'sidepanel.html';

chrome.runtime.onInstalled.addListener(() => {
    chrome.sidePanel.setPanelBehavior({openPanelOnActionClick: true});
});

chrome.action.onClicked.addListener((tab) => {
    if (!tab.url) return;
    const url = new URL(tab.url);
    if (url.origin === SITE_ORIGIN) {
        chrome.sidePanel.setOptions({tabId: tab.id, path: SIDEPANEL, enabled: true});
        chrome.sidePanel.open({tabId: tab.id});
    } else {
        chrome.sidePanel.setOptions({tabId: tab.id, enabled: false});
    }
});

chrome.commands.onCommand.addListener((command) => {
    // console.log(`Command: ${command}`);
});

chrome.runtime.onMessage.addListener(function (request, sender, sendResponse) {
    // for sendMessage in content-script.js
});
