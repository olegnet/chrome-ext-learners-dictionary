// noinspection DuplicatedCode

async function getCurrentTab() {
    const queryOptions = {active: true, currentWindow: true};
    const [tab] = await chrome.tabs.query(queryOptions);
    return tab;
}

function doDictionaryLookup(searchText) {
    window.document.getElementsByClassName('searchfield_input')[0].value = searchText;
    window.document.getElementById('search-form').submit();
}

function doOpenUrl(url) {
    if (window.location.href !== url) {
        window.location.href = url
    }
}

async function executeInCurrentTab(fn, args) {
    const tab = await getCurrentTab();
    await chrome.scripting.executeScript({
        target: {tabId: tab.id},
        func: fn,
        args: args
    });
}

export async function dictionaryLookup(searchText) {
    await executeInCurrentTab(doDictionaryLookup, [searchText]);
}

export async function openUrl(url) {
    await executeInCurrentTab(doOpenUrl, [url]);
}

// TODO avoid code duplication with content-script.js
async function doUpdateCurrentTabData() {
    let word = "";
    let wordClass = "";
    let title = "";
    let phonetics = "";
    try {
        if (window.location.href.split("/")[4] === "american_english") {
            for (let element of window.document.getElementsByClassName("webtop-g")) {
                if (element.nodeName === "DIV") {
                    word = element.getElementsByClassName("h")[0].firstChild.textContent
                    wordClass = element.getElementsByClassName("pos")[0].textContent;
                    break;
                }
            }
        } else {
            for (let element of window.document.getElementsByClassName("headword")) {
                if (element.nodeName === "H1") {
                    word = element.firstChild.textContent;
                    wordClass = element.parentElement.getElementsByClassName("pos")[0].textContent;
                    break;
                }
            }
            phonetics = window.document.getElementsByClassName("phonetics")[0]
                .getElementsByClassName("phons_br")[0]
                .getElementsByClassName("phon")[0]
                .textContent;
        }
    } catch (e) {
        // console.log(e);
    }
    try {
        title = window.document.head.getElementsByTagName("title")[0].textContent;
    } catch (e) {
        // console.log(e);
    }
    await chrome.runtime.sendMessage({
        from_content_script: "loaded",
        from_content_script_word: word,
        from_content_script_class: wordClass,
        from_content_script_title: title,
        from_content_script_phonetics: phonetics,
    });
}

export async function updateCurrentTabData() {
    await executeInCurrentTab(doUpdateCurrentTabData, []);
}

export function startDownload(url, filename) {
    chrome.downloads.download({
        url: url,
        filename: filename,
        saveAs: true
    });
}

export function scrollTo(id) {
    const container = document.getElementById("words-list");
    const target = document.getElementById(id);
    if (container && target) {
        container.scrollTo({
            top: target.offsetTop - container.offsetTop - 1,
            behavior: 'smooth'
        });
    }
}

export function doPlayPhonetics() {
    try {
        const element = window.document.getElementsByClassName("phonetics")[0]
            .getElementsByClassName("phons_br")[0]
            .getElementsByClassName("pron-uk")[0];
        const mp3Url = element.getAttribute('data-src-mp3');
        const oggUrl = element.getAttribute('data-src-ogg');
        const audio = new Audio(mp3Url || oggUrl);
        audio.play().catch(error => {
            // console.error('Audio playback failed: ', error);
        });
    } catch (e) {
        // console.log(e);
    }
}

export async function playPhonetics() {
    await executeInCurrentTab(doPlayPhonetics, []);
}
