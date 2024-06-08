/*
 * Copyright (c) 2024 Oleg Okhotnikov
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::fmt::{Display, Formatter};

use dioxus::prelude::*;
use dioxus_daisyui::prelude::*;
use dioxus_free_icons::Icon;
use dioxus_free_icons::icons::md_action_icons::MdSettings;
use dioxus_free_icons::icons::md_editor_icons::MdNotes;
use dioxus_free_icons::icons::md_file_icons::MdFolder;
use dioxus_free_icons::icons::md_navigation_icons::{MdArrowDropDown, MdArrowDropUp};
use dioxus_std::storage::{LocalStorage, use_synced_storage};
use futures_util::StreamExt;

use crate::model::{default_sort_direction, Folder, FolderKey, Word, WordKey};
use crate::storage_global::get_storage;
use crate::ui::{CURRENT_TAB_DATA, msg_select_folder_first, updateCurrentTabData};
use crate::ui::export_data::ExportData;
use crate::ui::folders::Folders;
use crate::ui::icons::EmptyIcon;
use crate::ui::import_data::ImportData;
use crate::ui::settings::Settings;
use crate::ui::show_copyright::ShowCopyright;
use crate::ui::words::Words;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Navigation {
    Folders,
    Words,
    Settings,
    ExportData,
    ImportData,
}

impl Display for Navigation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        // FIXME replace ":?"
        write!(f, "{:?}", self)
    }
}

impl From<String> for Navigation {
    fn from(str: String) -> Self {
        if Navigation::Folders.to_string() == str {
            Navigation::Folders
        } else if Navigation::Words.to_string() == str {
            Navigation::Words
        } else if Navigation::Settings.to_string() == str {
            Navigation::Settings
        } else if Navigation::ExportData.to_string() == str {
            Navigation::ExportData
        } else {
            Navigation::ImportData
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum DataProtection {
    Protected,
    Unprotected,
}

#[component]
pub fn Navigation() -> Element {
    let mut navigation_state = use_signal(|| Navigation::Folders);
    let mut navigation_error_str = use_signal(|| String::new());
    let navigation = use_coroutine(|mut rx| async move {
        while let Some(state) = rx.next().await {
            navigation_state.set(state);
            navigation_error_str.set(String::new());
        }
    });

    let folder_str = use_signal(|| String::new());
    let folder_note_str = use_signal(|| String::new());
    let folder_error_str = use_signal(|| String::new());

    let folders_page_length = use_synced_storage::<LocalStorage, Option<u32>>(
        "folders_page_length".to_string(), || None::<u32>);
    let folders_offset = use_signal(|| None::<u32>);
    let folders_direction = use_signal(|| default_sort_direction.to_string());
    let refresh_folders = use_signal(|| 0u8);

    let show_add_folder_form = use_synced_storage::<LocalStorage, u8>(
        "show_add_folder_form".to_string(), || 255u8);

    let selected_folder_str = use_synced_storage::<LocalStorage, String>(
        "selected_folder".to_string(), || String::new());
    let selected_folder_error_str = use_signal(|| String::new());
    let mut word_str = use_signal(|| String::new());
    let word_error_str = use_signal(|| String::new());
    let mut word_class_str = use_signal(|| String::new());
    let mut note_str = use_signal(|| String::new());

    let words_page_length = use_synced_storage::<LocalStorage, Option<u32>>(
        "page_length".to_string(), || None::<u32>);
    let words_page_offset = use_signal(|| None::<u32>);
    let words_direction = use_signal(|| default_sort_direction.to_string());
    let refresh_words = use_signal(|| 0u8);

    let show_add_word_form = use_synced_storage::<LocalStorage, u8>(
        "show_add_word_form".to_string(), || 255u8);

    let data_protection = use_signal(|| DataProtection::Protected);
    let data_protection_error = move ||
        navigation_error_str.set("Data protection is set. Check the settings to disable it".to_string());

    let _delete_word = use_coroutine(|mut rx: UnboundedReceiver<WordKey>| {
        to_owned![refresh_words, data_protection_error];
        async move {
            while let Some(word_key) = rx.next().await {
                match data_protection() {
                    // FIXME Find a better place for the messages
                    DataProtection::Protected => data_protection_error(),
                    DataProtection::Unprotected => {
                        let _ = get_storage().delete_by_id::<Word>(word_key.id).await;
                        refresh_words.toggle();
                        navigation_error_str.set("Word was deleted".to_string());
                    }
                }
            }
        }
    });

    let _delete_folder = use_coroutine(|mut rx: UnboundedReceiver<FolderKey>| {
        to_owned![refresh_folders, data_protection_error];
        async move {
            while let Some(folder_key) = rx.next().await {
                match data_protection() {
                    DataProtection::Protected => data_protection_error(),
                    DataProtection::Unprotected => {
                        let _ = get_storage().delete_by_id::<Folder>(folder_key.id).await;
                        refresh_folders.toggle();
                        navigation_error_str.set("Folder was deleted".to_string());
                    }
                }
            }
        }
    });

    let current_tab_data = use_memo(move || CURRENT_TAB_DATA());

    use_effect(move || {
        spawn(updateCurrentTabData());

        word_str.set(current_tab_data().word);
        word_class_str.set(current_tab_data().word_class);
        note_str.set(current_tab_data().phonetics);
    });

    rsx! {
        div { class: class!(text_lg),
            position: "relative",
            margin_top: "5px",
            form { action: "",
                onsubmit: move |event| event.stop_propagation(),
                label {
                    title: "Folders",
                    button { class: class!(btn btn_sm),
                        onclick: move |_| navigation.send(Navigation::Folders),
                        Icon {
                            fill: "black",
                            icon: MdFolder,
                        }
                        "Folders"
                    }
                }
                label {
                    title: "Show add folder form",
                    margin_left: "2px",
                    ShowFormButton {
                        navigation_state: navigation_state,
                        form_state: Navigation::Folders,
                        show_form: show_add_folder_form,
                    }
                }
                label {
                    margin_left: "5px",
                    title: "Words",
                    button { class: class!(btn btn_sm),
                        onclick: move |_| {
                            if selected_folder_str().len() != 0 {
                                navigation.send(Navigation::Words);
                            } else {
                                navigation_error_str.set(msg_select_folder_first.to_string());
                            }
                        },
                        Icon {
                            fill: "black",
                            icon: MdNotes,
                        }
                        "Words"
                    }
                }
                label {
                    title: "Show add word form",
                    margin_left: "2px",
                    ShowFormButton {
                        navigation_state: navigation_state,
                        form_state: Navigation::Words,
                        show_form: show_add_word_form,
                    }
                }
                label {
                    margin_left: "5px",
                    title: "Settings",
                    button { class: class!(btn btn_sm),
                        onclick: move |_| navigation.send(Navigation::Settings),
                        Icon {
                            fill: "black",
                            icon: MdSettings,
                        }
                    }
                }
                p { class: class!(text_xs text_red_500),
                    "{navigation_error_str}"
                }
            }
            match navigation_state() {
                Navigation::Folders => {
                    rsx! {
                        Folders {
                            folder_str: folder_str,
                            folder_note_str: folder_note_str,
                            folder_error_str: folder_error_str,
                            selected_folder_str: selected_folder_str,
                            page_length: folders_page_length,
                            offset: folders_offset,
                            direction: folders_direction,
                            refresh_folders: refresh_folders,
                            show_add_folder_form: show_add_folder_form,
                        }
                    }
                }
                Navigation::Words => {
                    rsx! {
                        Words {
                            selected_folder_str: selected_folder_str,
                            selected_folder_error_str: selected_folder_error_str,
                            word_str: word_str,
                            word_error_str: word_error_str,
                            word_class_str: word_class_str,
                            note_str: note_str,
                            page_length: words_page_length,
                            offset: words_page_offset,
                            direction: words_direction,
                            refresh_words: refresh_words,
                            show_add_word_form: show_add_word_form,
                        }
                    }
                }
                Navigation::Settings => {
                    rsx! {
                        Settings {
                            folders_page_length: folders_page_length,
                            folders_offset: folders_offset,
                            words_page_length: words_page_length,
                            words_offset: words_page_offset,
                            data_protection: data_protection,
                        }
                    }
                }
                Navigation::ExportData => {
                    rsx! {
                        ExportData {}
                    }
                }
                Navigation::ImportData => {
                    rsx! {
                        ImportData {}
                    }
                }
            }
        }
        if navigation_state() == Navigation::Settings {
            ShowCopyright {}
        }
    }
}

#[component]
pub fn ShowFormButton(
    navigation_state: Signal<Navigation>,
    form_state: Navigation,
    show_form: Signal<u8>,
) -> Element {
    let is_button_disabled = navigation_state() != form_state;
    let button_style = if is_button_disabled {
        "background-color: transparent; border-color: transparent;" // FIXME button's shadow
    } else { "" };

    rsx! {
        button { class: class!(btn btn_sm),
            style: "{button_style}",
            disabled: is_button_disabled,
            onclick: move |_| show_form.toggle(),
            if navigation_state() == form_state {
                if show_form() == 0u8 {
                    Icon {
                        fill: "black",
                        icon: MdArrowDropDown,
                    }
                } else {
                    Icon {
                        fill: "black",
                        icon: MdArrowDropUp,
                    }
                }
            } else {
                Icon {
                    fill: "black",
                    icon: EmptyIcon,
                }
            }
        }
    }
}
