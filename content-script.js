window.addEventListener("load",
    function () {
        (async () => {
            let word = "";
            let wordClass = "";
            let title = "";
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
                }
            } catch (e) {
                console.error(e);   // FIXME remove in prod
            }
            try {
                title = window.document.head.getElementsByTagName("title")[0].textContent;
            } catch (e) {
                console.error(e);   // FIXME remove in prod
            }
            await chrome.runtime.sendMessage({
                from_content_script: "loaded",
                from_content_script_word: word,
                from_content_script_class: wordClass,
                from_content_script_title: title,
            });
        })();
    },
    false
);
